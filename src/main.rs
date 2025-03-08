use actix::prelude::*;

#[actix::main]
async fn main() {
    println!("Hello, Actix!");

    let addr = State { i: 0 }.start();
    let res = addr.send(Ping(3)).await;
    println!("RES: {:?}", res);
    drop(addr);

    System::current().stop();
}


struct State {
    i: usize,
}

impl Actor for State {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("started ..");
    }
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!(".. stopped");
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for State {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!(".. pinged ..");
        msg.0
    }
}
