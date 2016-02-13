/*
 * chamkho - a word breaker written in Rust
 * Copyright (C) 2015  Vee Satayamas
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

use edge::{Edge,EdgeType};
use dict::Dict;
use acc::DictAcc;
use punc_acc::PuncAcc;

pub struct GraphBuilder<'a> {
    dict_acc: Vec<DictAcc>,
    punc_acc: PuncAcc,
    dict: &'a Dict,
    pub g: &'a mut Vec<Edge>,
    pub txt: &'a Vec<char>
}

fn best_edge(edges: &Vec<Edge>) -> Option<&Edge> {
    if edges.len() > 0 {
        let mut best = &edges[0];
        for i in 1..edges.len() {
            if !best.better_than(&edges[i]) {
                best = &edges[i];
            }
        }
        Some(best)
    } else {
        None
    }
}

impl<'a> GraphBuilder<'a> {
    pub fn new(txt: &'a Vec<char>, g: &'a mut  Vec<Edge>, dict: &'a Dict) -> GraphBuilder<'a> {
        g.push(Edge{w:0, unk:0, p: 0, etype: EdgeType::Init});
        GraphBuilder{dict_acc:vec![],
                     dict:dict,
                     g:g,
                     txt:txt,
                     punc_acc:PuncAcc::new()}
    }

    pub fn transit(&mut self, ch: char, nch: Option<char>) {
        self.transit_dict(ch);
        self.transit_space(ch, nch);
    }

    fn transit_dict(&mut self, ch: char) {
        self.dict_acc.push(DictAcc::new(0, self.dict.r()));
        self.dict_acc = self.dict_acc
            .iter()
            .map(|a| a.transit(ch, &self.dict))
            .filter(|t| t.is_some())
            .map(|t| t.unwrap())
            .collect()
    }

    fn transit_space(&mut self, ch: char, nch: Option<char>) {
        self.punc_acc.transit(ch, nch);
    }                   

    fn build_edges_from_dict(&mut self, i: usize) -> Vec<Edge> {
        self.dict_acc.iter()
            .filter(|a| a.is_final)
            .map(|a| {
                let p = 1 + i - a.offset;
                let src = &self.g[p];
                Edge{w:src.w+1, unk:src.unk, p:p, etype:EdgeType::Dict}
            }).collect()
    }           

    fn build_edges_unk(&mut self, left: usize) -> Vec<Edge> {
        let src = &self.g[left];
        vec![Edge{w:src.w+1, unk:src.unk+1, p:left, etype:EdgeType::Unk}]
    }

    fn build_edges_from_space(&mut self) -> Vec<Edge> {
        if self.punc_acc.is_space_final() {
            let src = &self.g[self.punc_acc.start];
            vec![Edge{w:src.w+1,
                      unk:src.unk,
                      p:self.punc_acc.start,
                      etype:EdgeType::Space}]
        } else {
            vec![]
        } 
    }
    
    fn build_edges(&mut self, i: usize, left: usize) -> Vec<Edge> {
        let dict_edges = self.build_edges_from_dict(i);
        if dict_edges.len() > 0 {
            return dict_edges
        } 

        let space_edges = self.build_edges_from_space();
        if space_edges.len() > 0 {
            return space_edges
        }
        
        return self.build_edges_unk(left)

    }

    pub fn build(&mut self) {
        let mut left = 0;
        let mut nch: Option<char> = if self.txt.len() > 0 {
            Some(self.txt[0])
        } else {
            None
        };

        let r = self.txt.len() - 1;
        
        for i in 0..r {
            let ch = nch.unwrap();
            nch = Some(self.txt[i+1]);
            self.transit(ch, nch);
            self.update_graph(i, &mut left);
        }

        // last char
        let ch = self.txt[r];
        self.transit(ch, None);
        self.update_graph(r, &mut left);
    }

    fn update_graph(&mut self, i: usize, left: &mut usize) {
        let edges = self.build_edges(i, *left);
        let e = best_edge(&edges).unwrap();
        let g = &mut self.g;
        let _e = e.clone();
        g.push(_e);
        if e.etype != EdgeType::Unk {
            *left = i + 1;
        }
    }              
}
