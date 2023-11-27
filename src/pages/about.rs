use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::{Handlebars, html_escape};

pub struct HandleAbout;

impl HandleAbout {
    pub async fn about(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the about page
        let about_content = json!({
            "content": html_escape("Welcome to my website! This space is where I get to share some of my favorite things with you — books that have left a lasting impact, music that resonates with my soul, and more.\n

            I find joy in the simple things — watching sunsets and cherishing moments with my family. Speaking of family, I'm married to my amazing husband, Serge. He's not just my partner in life but also my best friend and greatest motivator. Together, we navigate this beautiful journey, finding inspiration in each shared sunset and laughter.\n
            
            One of my greatest inspirations is Johannes Kepler. This German astronomer, mathematician, and astrologer played a pivotal role in the 17th-century scientific revolution. His laws of planetary motion have forever shaped our understanding of the cosmos. What captivates me most about Kepler is not just his groundbreaking contributions\n 
            but his humility in the face of monumental achievements. His modesty fuels my curiosity for space studies, and I aspire to approach my interests with the same sense of wonder and humility.\n
            
            Why I'm Here:
            I've created this blog as a space to share, and explore the things that bring joy to my life. Whether you're seeking book recommendations, musical discoveries, or just a glimpse into the everyday moments that inspire me, I hope you find something here that resonates with you.\n
            
            Thank you for joining me on this journey."),
        });
        let body =hb.render("about", &about_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}