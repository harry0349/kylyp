use diesel;
use diesel::prelude::*;
use model::article::{Article,Comment,NewArticle,NewComment};
use model::user::User;
use model::pg::get_conn;
use model::db::establish_connection;
use controller::user::UserId;
use chrono::prelude::*;
use easy::string::Htmlentities;

//trait Htmlentities {
//    fn html_encode(&mut self) -> String;
//}
//
//impl Htmlentities for String {
//    fn html_encode(&mut self) -> String {
//        self.replace("\"", "&quot;").replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace(" ", "&nbsp;").replace("¡", "&iexcl;").replace("¢", "&cent;").replace("£", "&pound;").replace("¤", "&curren;").replace("¥", "&yen;").replace("¦", "&brvbar;").replace("§", "&sect;").replace("¨", "&uml;").replace("©", "&copy;").replace("ª", "&ordf;").replace("«", "&laquo;").replace("¬", "&not;").replace("­", "&shy;").replace("®", "&reg;").replace("¯", "&macr;").replace("°", "&deg;").replace("±", "&plusmn;").replace("²", "&sup2;").replace("³", "&sup3;").replace("´", "&acute;").replace("µ", "&micro;").replace("¶", "&para;").replace("·", "&middot;").replace("¸", "&cedil;").replace("¹", "&sup1;").replace("º", "&ordm;").replace("»", "&raquo;").replace("¼", "&frac14;").replace("½", "&frac12;").replace("¾", "&frac34;").replace("¿", "&iquest;").replace("À", "&Agrave;").replace("Á", "&Aacute;").replace("Â", "&Acirc;").replace("Ã", "&Atilde;").replace("Ä", "&Auml;").replace("Å", "&Aring;").replace("Æ", "&AElig;").replace("Ç", "&Ccedil;").replace("È", "&Egrave;").replace("É", "&Eacute;").replace("Ê", "&Ecirc;").replace("Ë", "&Euml;").replace("Ì", "&Igrave;").replace("Í", "&Iacute;").replace("Î", "&Icirc;").replace("Ï", "&Iuml;").replace("Ð", "&ETH;").replace("Ñ", "&Ntilde;").replace("Ò", "&Ograve;").replace("Ó", "&Oacute;").replace("Ô", "&Ocirc;").replace("Õ", "&Otilde;").replace("Ö", "&Ouml;").replace("×", "&times;").replace("Ø", "&Oslash;").replace("Ù", "&Ugrave;").replace("Ú", "&Uacute;").replace("Û", "&Ucirc;").replace("Ü", "&Uuml;").replace("Ý", "&Yacute;").replace("Þ", "&THORN;").replace("ß", "&szlig;").replace("à", "&agrave;").replace("á", "&aacute;").replace("â", "&acirc;").replace("ã", "&atilde;").replace("ä", "&auml;").replace("å", "&aring;").replace("æ", "&aelig;").replace("ç", "&ccedil;").replace("è", "&egrave;").replace("é", "&eacute;").replace("ê", "&ecirc;").replace("ë", "&euml;").replace("ì", "&igrave;").replace("í", "&iacute;").replace("î", "&icirc;").replace("ï", "&iuml;").replace("ð", "&eth;").replace("ñ", "&ntilde;").replace("ò", "&ograve;").replace("ó", "&oacute;").replace("ô", "&ocirc;").replace("õ", "&otilde;").replace("ö", "&ouml;").replace("÷", "&divide;").replace("ø", "&oslash;").replace("ù", "&ugrave;").replace("ú", "&uacute;").replace("û", "&ucirc;").replace("ü", "&uuml;").replace("ý", "&yacute;").replace("þ", "&thorn;").replace("ÿ", "&yuml;").replace("Œ", "&OElig;").replace("œ", "&oelig;").replace("Š", "&Scaron;").replace("š", "&scaron;").replace("Ÿ", "&Yuml;").replace("ƒ", "&fnof;").replace("ˆ", "&circ;").replace("˜", "&tilde;").replace("Α", "&Alpha;").replace("Β", "&Beta;").replace("Γ", "&Gamma;").replace("Δ", "&Delta;").replace("Ε", "&Epsilon;").replace("Ζ", "&Zeta;").replace("Η", "&Eta;").replace("Θ", "&Theta;").replace("Ι", "&Iota;").replace("Κ", "&Kappa;").replace("Λ", "&Lambda;").replace("Μ", "&Mu;").replace("Ν", "&Nu;").replace("Ξ", "&Xi;").replace("Ο", "&Omicron;").replace("Π", "&Pi;").replace("Ρ", "&Rho;").replace("Σ", "&Sigma;").replace("Τ", "&Tau;").replace("Υ", "&Upsilon;").replace("Φ", "&Phi;").replace("Χ", "&Chi;").replace("Ψ", "&Psi;").replace("Ω", "&Omega;").replace("α", "&alpha;").replace("β", "&beta;").replace("γ", "&gamma;").replace("δ", "&delta;").replace("ε", "&epsilon;").replace("ζ", "&zeta;").replace("η", "&eta;").replace("θ", "&theta;").replace("ι", "&iota;").replace("κ", "&kappa;").replace("λ", "&lambda;").replace("μ", "&mu;").replace("ν", "&nu;").replace("ξ", "&xi;").replace("ο", "&omicron;").replace("π", "&pi;").replace("ρ", "&rho;").replace("ς", "&sigmaf;").replace("σ", "&sigma;").replace("τ", "&tau;").replace("υ", "&upsilon;").replace("φ", "&phi;").replace("χ", "&chi;").replace("ψ", "&psi;").replace("ω", "&omega;").replace("ϑ", "&thetasym;").replace("ϒ", "&upsih;").replace("ϖ", "&piv;").replace(" ", "&ensp;").replace(" ", "&emsp;").replace(" ", "&thinsp;").replace("‎", "&lrm;").replace("‏", "&rlm;").replace("–", "&ndash;").replace("—", "&mdash;").replace("‘", "&lsquo;").replace("’", "&rsquo;").replace("‚", "&sbquo;").replace("“", "&ldquo;").replace("”", "&rdquo;").replace("„", "&bdquo;").replace("†", "&dagger;").replace("‡", "&Dagger;").replace("•", "&bull;").replace("…", "&hellip;").replace("‰", "&permil;").replace("′", "&prime;").replace("″", "&Prime;").replace("‹", "&lsaquo;").replace("›", "&rsaquo;").replace("‾", "&oline;").replace("⁄", "&frasl;").replace("€", "&euro;").replace("ℑ", "&image;").replace("℘", "&weierp;").replace("ℜ", "&real;").replace("™", "&trade;").replace("ℵ", "&alefsym;").replace("←", "&larr;").replace("↑", "&uarr;").replace("→", "&rarr;").replace("↓", "&darr;").replace("↔", "&harr;").replace("↵", "&crarr;").replace("⇐", "&lArr;").replace("⇑", "&uArr;").replace("⇒", "&rArr;").replace("⇓", "&dArr;").replace("⇔", "&hArr;").replace("∀", "&forall;").replace("∂", "&part;").replace("∃", "&exist;").replace("∅", "&empty;").replace("∇", "&nabla;").replace("∈", "&isin;").replace("∉", "&notin;").replace("∋", "&ni;").replace("∏", "&prod;").replace("∑", "&sum;").replace("−", "&minus;").replace("∗", "&lowast;").replace("√", "&radic;").replace("∝", "&prop;").replace("∞", "&infin;").replace("∠", "&ang;").replace("∧", "&and;").replace("∨", "&or;").replace("∩", "&cap;").replace("∪", "&cup;").replace("∫", "&int;").replace("∴", "&there4;").replace("∼", "&sim;").replace("≅", "&cong;").replace("≈", "&asymp;").replace("≠", "&ne;").replace("≡", "&equiv;").replace("≤", "&le;").replace("≥", "&ge;").replace("⊂", "&sub;").replace("⊃", "&sup;").replace("⊄", "&nsub;").replace("⊆", "&sube;").replace("⊇", "&supe;").replace("⊕", "&oplus;").replace("⊗", "&otimes;").replace("⊥", "&perp;").replace("⋅", "&sdot;").replace("⌈", "&lceil;").replace("⌉", "&rceil;").replace("⌊", "&lfloor;").replace("⌋", "&rfloor;").replace("〈", "&lang;").replace("〉", "&rang;").replace("◊", "&loz;").replace("♠", "&spades;").replace("♣", "&clubs;").replace("♥", "&hearts;").replace("♦", "&diams;")
//    }
//}

#[derive(Debug, Serialize)]
pub struct Uarticle {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub content: String,
    pub createtime: String,
    pub updatetime: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct Ucomment {
    pub id: i32,
    pub aid: i32,
    pub uid: i32,
    pub content: String,
    pub createtime: String,
    pub username: String,
}
#[derive(Debug,Serialize)]
pub struct UserArticles<'a> {
    pub username: &'a str,
    pub title: &'a str,
    pub category: &'a str,
    pub createtime: &'a str,
    pub comments_count: i32,
}

pub fn article_list() -> Vec<Uarticle> {
    let conn = get_conn();
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn.query("SELECT article.*, users.username FROM article, users WHERE article.uid = users.id", &[]).unwrap()
        {
            let mut result = Uarticle {
                id: row.get(0),
                uid: row.get(1),
                category: row.get(2),
                status: row.get(3),
                comments_count: row.get(4),
                title: row.get(5),
                content: row.get(6),
                createtime: row.get(7),
                updatetime: row.get(8),
                username: row.get(9),
            };
            result.username = result.username.html_entities();
            result.content = result.content.html_entities();
            result.title = result.title.html_entities();
            article_result.push(result);
        }
    article_result
}


pub fn get_article_by_aid(aid: i32) -> Uarticle {
    let conn = get_conn();
    let mut article_result = Uarticle {
        id: 0,
        uid: 0,
        category: "".to_string(),
        status: 0,
        comments_count: 0,
        title: "".to_string(),
        content: "".to_string(),
        createtime: "".to_string(),
        updatetime: "".to_string(),
        username: "".to_string(),
    };
    for row in &conn.query("SELECT article.*, users.username FROM article, users WHERE article.uid = users.id and article.id = $1", &[&aid]).unwrap() {
        article_result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            content: row.get(6),
            createtime: row.get(7),
            updatetime: row.get(8),
            username: row.get(9),
        };
    }
    article_result.content = article_result.content.html_entities();
    article_result.title = article_result.title.html_entities();
    article_result.username = article_result.username.html_entities();
    article_result
}

pub fn get_comment_by_aid(aid: i32) -> Vec<Ucomment> {
    let conn = get_conn();
    let mut result: Vec<Ucomment> = vec![];
    for row in &conn.query("SELECT comment.*, users.username FROM comment, users WHERE comment.uid = users.id and comment.aid = $1 order by comment.id", &[&aid]).unwrap() {
        let mut comment_result = Ucomment {
            id: row.get(0),
            aid: row.get(1),
            uid: row.get(2),
            content: row.get(3),
            createtime: row.get(4),
            username: row.get(5),
        };
        comment_result.content = comment_result.content.html_entities();
        comment_result.username = comment_result.username.html_entities();
        result.push(comment_result);
    }
    result
}

pub fn add_article_by_uid<'a>(uid: i32, category: &'a str, title: &'a str, content: &'a str) {
    use utils::schema::article;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let updatetime = &Local::now().to_string();
    let new_article = NewArticle {
        uid,
        category,
        title,
        content,
        createtime,
        updatetime,
    };
    diesel::insert(&new_article).into(article::table).execute(&connection).expect("Error saving new list");
}

pub fn add_comment_by_aid<'a>(aid: i32, uid: i32, content: &'a str) {
    use utils::schema::comment;
    let connection = establish_connection();
    let createtime = &Local::now().to_string();
    let new_comment = NewComment {
        aid,
        uid,
        content,
        createtime,
    };
    diesel::insert(&new_comment).into(comment::table).execute(&connection).expect("Error saving new comment");
}

pub fn get_user_info(user_id: &UserId) -> Option<User> {
    use utils::schema::users::dsl::*;
    // use utils::schema::{users,article,comment,message};
    let connection = establish_connection();
    let uid = user_id.0;
    let user_result =  users.filter(&id.eq(&uid)).load::<User>(&connection);
    let login_user = match user_result {
        Ok(user_s) => match user_s.first() {
            Some(a_user) => Some(a_user.clone()),
            None => None,
        },
        Err(_) => None,
    };
    login_user
}

// pub fn get_user_articles(user_id: &UserId)  {
//     use utils::schema::article::dsl::*;
//     let connection = establish_connection();
//     let u_id = user_id.0;
//     let articles = article.filter(&uid.eq(&u_id)).load::<Article>(&connection);
    
// }