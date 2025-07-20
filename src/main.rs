use rust_learn::{Button, Screen, SelectBox};


fn main() {

    let scr = Screen {
        components: vec![
            Box::new(SelectBox{
                width: 80,
                height: 20,
                options: vec![
                    String::from("Male"),
                    String::from("Female"),
                    String::from("Other"),
                ]
            }),
            Box::new(Button {
                width: 20,
                height: 20,
                label: String::from("Submit")
            })
        ]
    };

    scr.run();

}
