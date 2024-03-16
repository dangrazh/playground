use chrono::{DateTime, Utc};
use quick_xml::events::{BytesText, Event};
use quick_xml::Reader;
use std::ffi::{OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

// #[path = "args.rs"]
// mod args;
// use args::*; //{ActionType, ExtractCommand, FilterCommand};

use crate::args::Arguments;
pub struct Parser<'a> {
    file_name: &'a PathBuf,
    file_path: &'a PathBuf,
    file_name_and_path: PathBuf,
    action: &'a super::args::ActionType,
    outfile_name: PathBuf,
    outfile_writer: BufWriter<File>,
}

impl<'a> Parser<'a> {
    pub fn new(cli: &'a Arguments) -> Self {
        // filename: &'a PathBuf, path: &'a PathBuf, action: &'a ActionType
        let mut filename_path = PathBuf::new();
        filename_path.push(&cli.path);
        filename_path.push(&cli.filename);

        // create the output filename
        let mut fileout = PathBuf::new();
        let fileout_default_stem = OsString::from("outputfile".to_string());
        let fileout_default_ext = OsString::from("xml".to_string());
        fileout.push(&cli.path);
        let tmstmp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let mut fileout_file_stem = cli
            .filename
            .file_stem()
            .unwrap_or(&fileout_default_stem)
            .to_owned();

        match &cli.action as &super::args::ActionType {
            super::args::ActionType::Extract(_ec) => {
                fileout_file_stem.push("_extract_");
                fileout_file_stem.push(tmstmp);
                fileout.push(fileout_file_stem);
                fileout.set_extension("csv");
            }
            super::args::ActionType::Filter(_fc) => {
                fileout_file_stem.push("_filtered_");
                fileout_file_stem.push(tmstmp);
                fileout.push(fileout_file_stem);
                fileout.set_extension(cli.filename.extension().unwrap_or(&fileout_default_ext));
            }
        }
        println!("creating output file: {}", &fileout.to_string_lossy());
        let f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&fileout)
            .expect("Unable to open file");
        let bf_wr = BufWriter::new(f);

        Parser {
            file_name: &cli.filename,
            file_path: &cli.path,
            file_name_and_path: filename_path,
            action: &cli.action as &super::args::ActionType,
            outfile_name: fileout,
            outfile_writer: bf_wr,
        }
    }
}

impl Parser<'_> {
    pub fn process_file(&mut self) {
        let mut content = String::new();

        let filename = String::from(self.file_name_and_path.to_string_lossy().to_owned());
        println!("Reading file: {}", &filename);

        // Open the file in read-only mode.
        match File::open(&filename) {
            // The file is open (no error).
            Ok(mut file) => {
                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();
            }
            // Error handling.
            Err(error) => {
                panic!("Error opening file {}: {}", &filename, error);
            }
        }
        println!("File read into memory");
        match self.action {
            super::ActionType::Extract(super::ExtractCommand { tags }) => {
                self.extract_xml_tags(&content, tags).unwrap()
            }
            super::ActionType::Filter(super::FilterCommand { criteria, relation }) => {
                println!("Apologies, filtering is still under construction!")
                // self.filter_xml(&content, criteria, relation).unwrap()
            }
        }
    }

    fn extract_xml_tags(
        &mut self,
        xml: &str,
        taglist: &Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // setup reader and processing flags
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        reader.trim_text(true);

        let mut no_of_tags = 0;
        let mut no_of_ntry = 0;
        let mut elname: String;
        let mut process_tag: bool = false;
        let mut tag_idx: usize = 0;
        let ntry: String = String::from("Ntry");

        // construct and output the header
        println!("Exctracting tags: {}", &taglist.join(", "));
        let mut out_header = taglist.join(";");
        out_header.push_str("\n");
        self.write_record(&out_header);

        // construct the output record
        let mut out_rec: Vec<String> = Vec::with_capacity(taglist.len());
        for _ in taglist {
            out_rec.push("".to_string())
        }

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    // increase the tag count
                    no_of_tags += 1;
                    // get the element name
                    elname = String::from_utf8_lossy(e.name().into_inner()).to_string();
                    // if elname is in list of provided tags, flag element for processing
                    for (idx, elem) in taglist.iter().enumerate() {
                        if elname.eq(elem) {
                            // print!("tag found: {}", elem);
                            process_tag = true;
                            tag_idx = idx;
                            break;
                        }
                    }
                }
                Ok(Event::Text(ref e)) => {
                    // process the Tag Content
                    if process_tag {
                        let tag_value = e
                            .unescape()
                            .unwrap_or(std::borrow::Cow::from("no text"))
                            .to_string();

                        // print!("adding value: {}", &tag_value);
                        out_rec[tag_idx] = tag_value;
                    }
                }
                Ok(Event::CData(e)) => {
                    // process the Tag Content
                    if process_tag {
                        let tag_value = e
                            .escape()
                            .unwrap_or(BytesText::new("no text".as_ref()))
                            .unescape()
                            .unwrap_or(std::borrow::Cow::from("no text"))
                            .to_string();

                        // print!("adding value: {}", &tag_value);
                        out_rec[tag_idx] = tag_value;
                    }
                }
                Ok(Event::Empty(_e)) => {} //no need to process empty elements
                Ok(Event::Comment(_e)) => {} //no need to process empty elements
                // Ok(Event::CData(_e)) => {}
                Ok(Event::Decl(_e)) => {}
                Ok(Event::PI(_e)) => {} //no need to process processing instructions
                Ok(Event::DocType(_e)) => {}
                Ok(Event::End(e)) => {
                    // Tag End reached

                    // if we are closing a 'Ntry' tag, process the data
                    let close_tag = String::from_utf8_lossy(e.name().into_inner()).to_string();
                    if close_tag.eq(&ntry) {
                        no_of_ntry += 1;
                        let mut out_data = out_rec.join(";");
                        out_data.push_str("\n");
                        self.write_record(&out_data);
                    }

                    // do cleanup work as needed
                    process_tag = false;
                    tag_idx = 0;
                    // for el in out_rec.iter_mut() {
                    //     *el = "".to_string();
                    // }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => {
                    // return an error
                    let msg = format!("Error at position {}: {:?}", reader.buffer_position(), e);
                    return Err(msg)?;
                } // _ => (), // All `Event`s are handled above
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        println!(
            "Processing complete. Processed {} tags and {} 'Ntry' elements.",
            no_of_tags, no_of_ntry
        );

        Ok(())
    }

    fn filter_xml(
        self,
        xml: &str,
        criteria: &Vec<(String, String)>,
        relation: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut elname: String;

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    // get the element name
                    elname = String::from_utf8_lossy(e.name().into_inner()).to_string();
                }
                Ok(Event::Text(ref e)) => {
                    // process the Tag Content
                }
                Ok(Event::CData(ref e)) => {
                    // process the Tag Content
                }
                Ok(Event::Empty(_e)) => {} //no need to process empty elements
                Ok(Event::Comment(_e)) => {} //no need to process empty elements
                // Ok(Event::CData(_e)) => {}
                Ok(Event::Decl(_e)) => {}
                Ok(Event::PI(_e)) => {} //no need to process processing instructions
                Ok(Event::DocType(_e)) => {}
                Ok(Event::End(_e)) => {
                    // Tag End reached
                    // do cleanup work as needed
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => {
                    // return an error
                    let msg = format!("Error at position {}: {:?}", reader.buffer_position(), e);
                    return Err(msg)?;
                } // _ => (), // All `Event`s are handled above
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        Ok(())
    }

    #[inline]
    fn write_record(&mut self, data: &String) {
        // println!("{}", &data);
        self.outfile_writer
            .write_all(data.as_bytes())
            .expect("Unable to write data");
    }
}
