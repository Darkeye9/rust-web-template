use iron::{BeforeMiddleware, AfterMiddleware, status};
use iron::prelude::*;

pub struct ErrorHandler;

impl BeforeMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        // We can use the IronError from previous middleware to decide what to do.
        // Returning Ok() from a catch method resumes the normal flow and
        // passes the Request forward to the next middleware piece in the chain (here the HelloWorldHandler).
        println!("{} caught in ErrorRecover BeforeMiddleware.", err.error);
        match err.response.status {
            Some(status::BadRequest) => Ok(()),
            _ => Err(err)
        }
    }
}

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        // Just like in the BeforeMiddleware, we can return Ok(Response) here to return to the normal flow.
        // In this case, ErrorRecover is the last middleware in the chain
        // and the Response created in the ErrorProducer is modified and sent back to the client.
        println!("{} caught in ErrorRecover AfterMiddleware.", err.error);
        match err.response.status {
            Some(status::BadRequest) => Ok(err.response.set(status::Ok)),
            _ => Err(err)
        }
    }
}
