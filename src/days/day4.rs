use std::io::{BufReader, Read, BufRead};

#[derive(PartialEq, Debug)]
pub struct SectionBounds {
    pub left: i32,
    pub right: i32
}

pub fn get_section_bounds(section_bounds_pair: &str) -> Vec<SectionBounds> {
    return section_bounds_pair.split(',')
                              .map(|bounds| bounds.split('-')
                                                        .map(|bound| bound.parse::<i32>().unwrap()))
                              .map(|mut bounds| SectionBounds { left: bounds.next().unwrap(), right: bounds.next().unwrap() })
                              .collect();
}

pub fn has_fully_contained_section(section_bounds: Vec<SectionBounds>) -> bool {
    for i in 0..section_bounds.len() - 1 {
        for j in i + 1..section_bounds.len() {
            if (section_bounds[i].left >= section_bounds[j].left && section_bounds[i].right <= section_bounds[j].right) ||
               (section_bounds[j].left >= section_bounds[i].left && section_bounds[j].right <= section_bounds[i].right) {
                return true;
            }
        }
    }

    return false;
}

pub fn do_sections_overlap(a: &SectionBounds, b: &SectionBounds) -> bool {
    return !(a.left > b.right || a.right < b.left);
}

pub fn count_fully_contained_sections<R>(reader: &mut BufReader<R>) -> i32 where R: Read {
    let mut fully_contained_sections = 0;
    for line in reader.lines() {
        let bounds = get_section_bounds(&line.unwrap());
        if has_fully_contained_section(bounds) {
            fully_contained_sections += 1;
        }
    }

    return fully_contained_sections;
}

pub fn count_overlapping_sections<R>(reader: &mut BufReader<R>) -> i32 where R: Read {
    let mut overlapping_sections = 0;
    for line in reader.lines() {
        let bounds = get_section_bounds(&line.unwrap());
        if do_sections_overlap(&bounds[0], &bounds[1]) {
            overlapping_sections += 1;
        }
    }

    return overlapping_sections;
}