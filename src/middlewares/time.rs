extern crate time;

use self::time::precise_time_ns;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::{Request, Response, IronResult, Set};
use iron::headers::ContentType;
use iron::response::{ResponseBody, WriteBody};
use std::io::Cursor;

pub struct ResponseTime;

impl typemap::Key for ResponseTime {
    type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {


        if res.headers.has::<ContentType>() {
            if res.headers.get::<ContentType>().unwrap() == &ContentType::html() {
                // What the hell is required to do to get the response body, LOL!
                let mut buff = Cursor::new(Vec::new());
                {
                    let mut new_response = ResponseBody::new(buff.get_mut());
                    let body_writer: &mut Box<WriteBody> = res.body.as_mut().unwrap();
                    body_writer.write_body(&mut new_response).unwrap();
                }
                let mut body_text = String::from_utf8(buff.into_inner()).unwrap();

                let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
                let ms = (delta as f64) / 1000000.0;

                body_text = body_text.replace("@proc_time@", format!("{:.2}", ms).as_str());
                res.set_mut(body_text);

                // println!("Request took: {:?} ms", ms);
            }

        }

        Ok(res)
    }
}
