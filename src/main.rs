use typestate_builder::error::Result;
use typestate_builder::HTTPMethod;
use typestate_builder::RequestBuilder;

fn main() -> Result<()> {
    let req_builder = RequestBuilder::new()
        .url("https://some-url.com") // Comment this line for compiler error
        .method(HTTPMethod::Get);

    let req_builder = req_builder.header("Token", "somethig").seal(); // Without the seal, we cannot build

    let req1 = req_builder.clone().build();
    println!("{:#?}", req1);

    // let req2 = req_builder.method("POST").build()?; Won't compile due to seal
    let req2 = req_builder.build();
    println!("{:#?}", req2);

    Ok(())
}
