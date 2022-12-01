use reqwest::{
    header::HeaderMap,
    header::{HeaderValue, SET_COOKIE},
    redirect::{self, Policy},
    Body,
};
use std::{collections::HashMap, error::Error};

//Patch
pub struct Patch {
    url: String,
    acting: String,
    path: String,
    time_out: String,
    jump: bool,
    data: String,
    header: String,
}



//实现Patch请求

impl Patch {
    pub fn new(_url:&str,_acting:&str,_path:&str,_time_out:&str,_jump:bool,_data:&str,_header:&str) -> Patch {
        return Patch { 
            url: _url.to_string(),
            acting: _acting.to_string(), 
            path: _path.to_string(), 
            time_out: _time_out.to_string(), 
            jump: _jump, 
            data: _data.to_string(),
            header: _header.to_string(),};
    }

    pub async fn out(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        //获取用户输入的Header,转换成HeaderMap，返回
        fn request_header(test: String) -> HeaderMap {
            //let mut map: HeaderMap<&str> = HeaderMap::default();
            let split = test.split("\n"); //根据换行符分割
            let mut map = reqwest::header::HeaderMap::new();

            let mut headers1 = vec![];

            for k in split {
                //headers1.push(&k[0..(k.find(":")).unwrap()]);

                if !(k.is_empty()) {
                    headers1.push(k);
                }
            }

            let headers = &headers1;
 

            for &header in headers {
                //let counter = map.entry(header).or_insert("");

                let key = &header[0..(&header.find(":")).unwrap()];
                let value = &header[(&header.find(":")).unwrap() + 1..(header.len())];

       

                map.insert(
                    match reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                        Ok(d) => d,
                        Err(e) => {
                            println!("{:?}", e);
                            return map;
                        }
                    },
                    match reqwest::header::HeaderValue::from_str(value) {
                        Ok(d) => d,
                        Err(e) => {
                            println!("{:?}", e);
                            return map;
                        }
                    },
                );
            }

      

            map
        }

        //判断是否开启重定向
        fn custom(test: bool) -> Policy {


            let customa = redirect::Policy::custom(move |attempt| {
                if test {
                    if attempt.previous().len() > 5 {
                        attempt.error("too many redirects")
                    } else if attempt.url().host_str() == Some("example.domain") {
                        // prevent redirects to 'example.domain'
                        attempt.stop()
                    } else {
                        attempt.follow()
                    }
                } else {
                    attempt.stop()
                }
            });
            customa
        }

        //开启代理版本
        async fn send_acting(
            data: &Patch,
        ) -> Result<HashMap<String, String>, Box<dyn Error>> {
            //开始发包
            let client = reqwest::Client::builder()
                .gzip(true)
                .proxy(match reqwest::Proxy::all(&data.acting) {
                    Ok(d) => d,
                    Err(e) => {
                        println!("{:?}", e);
                        return Ok(HashMap::new());
                    }
                })
                .redirect(custom(data.jump))
                .build()?;

            let mut _data: HashMap<String, String> = HashMap::new();
            let res = client
                .patch(&data.url)
                .headers(request_header(data.header.to_string()))
                .body(data.data.clone())
                .send()
                .await?;

            let mut head = String::new();
            let test = res.headers();

            for (key, value) in test {
                
                let value1 = match value.to_str() {
                    Ok(cess) => cess,
                    Err(_) => todo!(),
                };

                match key {
                    _ => {
                        head += &((key.to_string() + &String::from(":") + &(value1).to_string())
                            + &"\n".to_string());
                    }
                };

                //head += &key.expect("msg").to_string();
                //head += &(value1).to_string();
            }

            //得到Cookie,类型为 &HeaderValue
            let cookie = res.headers().get(SET_COOKIE);

            match cookie {
                Option::Some(val) => val,
                Option::None => &HeaderValue::from_static(""),
            };

            // &HeaderValue转换String
            let cookie2 = &match cookie {
                Some(c) => c.to_str().unwrap_or_default().to_string(),
                None => String::from(""),
            };

            let html = &res.text().await?;

            _data.insert(String::from("html"), (&html).to_string());
            _data.insert(String::from("head"), head);
            _data.insert(String::from("Cookie"), (&cookie2).to_string());
            return Ok(_data);
        }

        async fn send(
            data: &Patch,
        ) -> Result<HashMap<String, String>, Box<dyn Error>> {
            //开始发包
            let client = reqwest::Client::builder()
                .gzip(true)
                .redirect(custom(data.jump))
                .build()?;

            let mut _data: HashMap<String, String> = HashMap::new();

            let res = client
                .patch(&data.url)
                .headers(request_header(data.header.to_string()))
                .body(data.data.clone())
                .send()
                .await?;

            let mut head = String::new();
            let test = res.headers();

            for (key, value) in test {
                
                let value1 = match value.to_str() {
                    Ok(cess) => cess,
                    Err(_) => todo!(),
                };

                match key {
                    _ => {
                        head += &((key.to_string() + &String::from(":") + &(value1).to_string())
                            + &"\n".to_string());
                    }
                };

                //head += &key.expect("msg").to_string();
                //head += &(value1).to_string();
            }

            //得到Cookie,类型为 &HeaderValue
            let cookie = res.headers().get(SET_COOKIE);

            match cookie {
                Option::Some(val) => val,
                Option::None => &HeaderValue::from_static(""),
            };

            // &HeaderValue转换String

            let cookie2 = &match cookie {
                Some(c) => c.to_str().unwrap_or_default().to_string(),
                None => String::from(""),
            };

            let html = &res.text().await?;

            _data.insert(String::from("Cookie"), (&cookie2).to_string());
            _data.insert(String::from("html"), (&html).to_string());
            _data.insert(String::from("head"), head);

            Ok(_data)
        }




        fn enable_proxy(test: String) -> bool {
            if !test.is_empty() {
                return true;
            } else {
                return false;
            }
        }

        //判断是否开启代理
        if enable_proxy(self.acting.clone()) {
            let _data = send_acting(&self).await?;
            
            Ok(_data)
        } else {
            let _data = send(&self).await?;
            Ok(_data)
        }
    }
}
