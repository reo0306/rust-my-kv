use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    city: String,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .post_async("/:country", |mut req, ctx| async move {
            let country = ctx.param("country").unwrap();

            let city = match req.json::<Country>().await {
                Ok(c) => c.city,
                Err(_) => String::from("")

            };

            if city.is_empty() {
                return Response::error("Bad Request", 400);
            };

            match ctx.kv("cities")?.put(country, &city)?.execute().await {
                Ok(_) => Response::ok(city),
                Err(_) => Response::error("Bad Request", 400),
            }
        })
        .get_async("/:country", |_req, ctx| async move {
            if let Some(country) = ctx.param("country") {
                return match ctx.kv("cities")?.get(country).text().await? {
                    Some(city) => Response::ok(city),
                    None => Response::error("Country not found", 404),
                };
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
