#![feature(coverage_attribute)]
#![feature(noop_waker)]
// edition: 2018

macro_rules! bail {
    ($msg:literal $(,)?) => {
        if $msg.len() > 0 {
            println!("no msg");
        } else {
            println!($msg);
        }
        return Err(String::from($msg));
    };
}

macro_rules! on_error {
    ($value:expr, $error_message:expr) => {
        $value.or_else(|e| { // FIXME(85000): no coverage in closure macros
            let message = format!($error_message, e);
            if message.len() > 0 {
                println!("{}", message);
                Ok(String::from("ok"))
            } else {
                bail!("error");
            }
        })
    };
}

fn load_configuration_files() -> Result<String, String> {
    Ok(String::from("config"))
}

pub async fn test() -> Result<(), String> {
    println!("Starting service");
    let config = on_error!(load_configuration_files(), "Error loading configs: {}")?;

    let startup_delay_duration = String::from("arg");
    let _ = (config, startup_delay_duration);
    Ok(())
}

#[coverage(off)]
fn main() {
    executor::block_on(test()).unwrap();
}

mod executor {
    use core::future::Future;
    use core::pin::pin;
    use core::task::{Context, Poll, Waker};

    #[coverage(off)]
    pub fn block_on<F: Future>(mut future: F) -> F::Output {
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());

        loop {
            if let Poll::Ready(val) = future.as_mut().poll(&mut context) {
                break val;
            }
        }
    }
}
