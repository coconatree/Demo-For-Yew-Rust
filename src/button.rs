use yew::prelude::*;

#[allow(dead_code)]
pub enum Msg {
    Add(f32),
    Sub(f32),
    Mul(f32),
    Div(f32),
}

pub struct ButtonPanel {
    amount: f32,
} 

impl Component for ButtonPanel {
    type Message    = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        println!("Creating the custom button component !!!");
        ButtonPanel {
            amount: 0.0,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(val) => {
                println!("Add has been called !!!");
                self.amount  += val;
                true
            },
            Msg::Sub(val) => { 
                println!("Sub had been called !!!");
                self.amount -= val;
                true 
            },
            Msg::Mul(val) => { 
                println!("Mul has been called !!!");
                self.amount *= val;
                true 
            },
            Msg::Div(val) => { 
                println!("Div has benn called !!!");
                self.amount /= val;
                true 
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let onclick_add = ctx.link().callback(|_| Msg::Add(5.0));
        let onclick_sub = ctx.link().callback(|_| Msg::Sub(5.0));
        let onclick_div = ctx.link().callback(|_| Msg::Div(5.0));
        let onclick_mul = ctx.link().callback(|_| Msg::Mul(5.0));

        html! {
            <div class = "container">
                <div class = "upper_container">
                    <p class = "text"> { self.amount } </p>
                </div>
                <div class = "lower_container">
                    <button class = "btn" onclick = {onclick_add} >{ "Add Button" }</button>
                    <button class = "btn" onclick = {onclick_sub} >{ "Sub Button" }</button>
                    <button class = "btn" onclick = {onclick_div} >{ "Div Button" }</button>
                    <button class = "btn" onclick = {onclick_mul} >{ "Mul Button" }</button>
                </div>
            </div>
        }
    }
}
