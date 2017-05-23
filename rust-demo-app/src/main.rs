extern crate iron;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use std::env;
use std::u32;
use std::collections::HashMap;
use std::str::FromStr;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use iron::{Iron, Request, Response, IronResult};
//use iron::request::Request;


struct Pokemon {
    number: u32,
    number_string: String,
    name: String
}




fn pokemon()->Pokemon  {
  let mut pokemons = HashMap::new();

  pokemons.insert("001","bulbasaur");
  pokemons.insert("022","fearow");
  pokemons.insert("043","oddish");
  pokemons.insert("064","kadabra");
  pokemons.insert("085","dodrio");
  pokemons.insert("106","hitmonlee");
  pokemons.insert("127","pinsir");
  pokemons.insert("148","dragonair");
  pokemons.insert("169","crobat");
  pokemons.insert("190","aipom");
  pokemons.insert("211","qwilfish");
  pokemons.insert("232","donphan");
  pokemons.insert("002","ivysaur");
  pokemons.insert("023","ekans");
  pokemons.insert("044","gloom");
  pokemons.insert("065","alakazam");
  pokemons.insert("086","seel");
  pokemons.insert("107","hitmonchan");
  pokemons.insert("128","tauros");
  pokemons.insert("149","dragonite");
  pokemons.insert("170","chinchou");
  pokemons.insert("191","sunkern");
  pokemons.insert("212","scizor");
  pokemons.insert("233","porygon2");
  pokemons.insert("003","venusaur");
  pokemons.insert("024","arbok");
  pokemons.insert("045","vileplume");
  pokemons.insert("066","machop");
  pokemons.insert("087","dewgong");
  pokemons.insert("108","lickitung");
  pokemons.insert("129","magikarp");
  pokemons.insert("150","mewtwo");
  pokemons.insert("171","lanturn");
  pokemons.insert("192","sunflora");
  pokemons.insert("213","shuckle");
  pokemons.insert("234","stantler");
  pokemons.insert("004","charmander");
  pokemons.insert("025","pikachu");
  pokemons.insert("046","paras");
  pokemons.insert("067","machoke");
  pokemons.insert("088","grimer");
  pokemons.insert("109","koffing");
  pokemons.insert("130","gyarados");
  pokemons.insert("151","mew");
  pokemons.insert("172","pichu");
  pokemons.insert("193","yanma");
  pokemons.insert("214","heracross");
  pokemons.insert("235","smeargle");
  pokemons.insert("005","charmeleon");
  pokemons.insert("026","raichu");
  pokemons.insert("047","parasect");
  pokemons.insert("068","machamp");
  pokemons.insert("089","muk");
  pokemons.insert("110","weezing");
  pokemons.insert("131","lapras");
  pokemons.insert("152","chikorita");
  pokemons.insert("173","cleffa");
  pokemons.insert("194","wooper");
  pokemons.insert("215","sneasel");
  pokemons.insert("236","tyrogue");
  pokemons.insert("006","charizard");
  pokemons.insert("027","sandshrew");
  pokemons.insert("048","venonat");
  pokemons.insert("069","bellsprout");
  pokemons.insert("090","shellder");
  pokemons.insert("111","rhyhorn");
  pokemons.insert("132","ditto");
  pokemons.insert("153","bayleef");
  pokemons.insert("174","igglybuff");
  pokemons.insert("195","quagsire");
  pokemons.insert("216","teddiursa");
  pokemons.insert("237","hitmontop");
  pokemons.insert("007","squirtle");
  pokemons.insert("028","sandslash");
  pokemons.insert("049","venomoth");
  pokemons.insert("070","weepinbell");
  pokemons.insert("091","cloyster");
  pokemons.insert("112","rhydon");
  pokemons.insert("133","eevee");
  pokemons.insert("154","meganium");
  pokemons.insert("175","togepi");
  pokemons.insert("196","espeon");
  pokemons.insert("217","ursaring");
  pokemons.insert("238","smoochum");
  pokemons.insert("008","wartortle");
  pokemons.insert("029","nidoran-f");
  pokemons.insert("050","diglett");
  pokemons.insert("071","victreebel");
  pokemons.insert("092","gastly");
  pokemons.insert("113","chansey");
  pokemons.insert("134","vaporeon");
  pokemons.insert("155","cyndaquil");
  pokemons.insert("176","togetic");
  pokemons.insert("197","umbreon");
  pokemons.insert("218","slugma");
  pokemons.insert("239","elekid");
  pokemons.insert("009","blastoise");
  pokemons.insert("030","nidorina");
  pokemons.insert("051","dugtrio");
  pokemons.insert("072","tentacool");
  pokemons.insert("093","haunter");
  pokemons.insert("114","tangela");
  pokemons.insert("135","jolteon");
  pokemons.insert("156","quilava");
  pokemons.insert("177","natu");
  pokemons.insert("198","murkrow");
  pokemons.insert("219","magcargo");
  pokemons.insert("240","magby");
  pokemons.insert("010","caterpie");
  pokemons.insert("031","nidoqueen");
  pokemons.insert("052","meowth");
  pokemons.insert("073","tentacruel");
  pokemons.insert("094","gengar");
  pokemons.insert("115","kangaskhan");
  pokemons.insert("136","flareon");
  pokemons.insert("157","typhlosion");
  pokemons.insert("178","xatu");
  pokemons.insert("199","slowking");
  pokemons.insert("220","swinub");
  pokemons.insert("241","miltank");
  pokemons.insert("011","metapod");
  pokemons.insert("032","nidoran-m");
  pokemons.insert("053","persian");
  pokemons.insert("074","geodude");
  pokemons.insert("095","onix");
  pokemons.insert("116","horsea");
  pokemons.insert("137","porygon");
  pokemons.insert("158","totodile");
  pokemons.insert("179","mareep");
  pokemons.insert("200","misdreavus");
  pokemons.insert("221","piloswine");
  pokemons.insert("242","blissey");
  pokemons.insert("012","butterfree");
  pokemons.insert("033","nidorino");
  pokemons.insert("054","psyduck");
  pokemons.insert("075","graveler");
  pokemons.insert("096","drowzee");
  pokemons.insert("117","seadra");
  pokemons.insert("138","omanyte");
  pokemons.insert("159","croconaw");
  pokemons.insert("180","flaaffy");
  pokemons.insert("201","unown");
  pokemons.insert("222","corsola");
  pokemons.insert("243","raikou");
  pokemons.insert("013","weedle");
  pokemons.insert("034","nidoking");
  pokemons.insert("055","golduck");
  pokemons.insert("076","golem");
  pokemons.insert("097","hypno");
  pokemons.insert("118","goldeen");
  pokemons.insert("139","omastar");
  pokemons.insert("160","feraligatr");
  pokemons.insert("181","ampharos");
  pokemons.insert("202","wobbuffet");
  pokemons.insert("223","remoraid");
  pokemons.insert("244","entei");
  pokemons.insert("014","kakuna");
  pokemons.insert("035","clefairy");
  pokemons.insert("056","mankey");
  pokemons.insert("077","ponyta");
  pokemons.insert("098","krabby");
  pokemons.insert("119","seaking");
  pokemons.insert("140","kabuto");
  pokemons.insert("161","sentret");
  pokemons.insert("182","bellossom");
  pokemons.insert("203","girafarig");
  pokemons.insert("224","octillery");
  pokemons.insert("245","suicune");
  pokemons.insert("015","beedrill");
  pokemons.insert("036","clefable");
  pokemons.insert("057","primeape");
  pokemons.insert("078","rapidash");
  pokemons.insert("099","kingler");
  pokemons.insert("120","staryu");
  pokemons.insert("141","kabutops");
  pokemons.insert("162","furret");
  pokemons.insert("183","marill");
  pokemons.insert("204","pineco");
  pokemons.insert("225","delibird");
  pokemons.insert("246","larvitar");
  pokemons.insert("016","pidgey");
  pokemons.insert("037","vulpix");
  pokemons.insert("058","growlithe");
  pokemons.insert("079","slowpoke");
  pokemons.insert("100","voltorb");
  pokemons.insert("121","starmie");
  pokemons.insert("142","aerodactyl");
  pokemons.insert("163","hoothoot");
  pokemons.insert("184","azumarill");
  pokemons.insert("205","forretress");
  pokemons.insert("226","mantine");
  pokemons.insert("247","pupitar");
  pokemons.insert("017","pidgeotto");
  pokemons.insert("038","ninetales");
  pokemons.insert("059","arcanine");
  pokemons.insert("080","slowbro");
  pokemons.insert("101","electrode");
  pokemons.insert("122","mrmime");
  pokemons.insert("143","snorlax");
  pokemons.insert("164","noctowl");
  pokemons.insert("185","sudowoodo");
  pokemons.insert("206","dunsparce");
  pokemons.insert("227","skarmory");
  pokemons.insert("248","tyranitar");
  pokemons.insert("018","pidgeot");
  pokemons.insert("039","jigglypuff");
  pokemons.insert("060","poliwag");
  pokemons.insert("081","magnemite");
  pokemons.insert("102","exeggcute");
  pokemons.insert("123","scyther");
  pokemons.insert("144","articuno");
  pokemons.insert("165","ledyba");
  pokemons.insert("186","politoed");
  pokemons.insert("207","gligar");
  pokemons.insert("228","houndour");
  pokemons.insert("249","lugia");
  pokemons.insert("019","rattata");
  pokemons.insert("040","wigglytuff");
  pokemons.insert("061","poliwhirl");
  pokemons.insert("082","magneton");
  pokemons.insert("103","exeggutor");
  pokemons.insert("124","jynx");
  pokemons.insert("145","zapdos");
  pokemons.insert("166","ledian");
  pokemons.insert("187","hoppip");
  pokemons.insert("208","steelix");
  pokemons.insert("229","houndoom");
  pokemons.insert("250","ho-oh");
  pokemons.insert("020","raticate");
  pokemons.insert("041","zubat");
  pokemons.insert("062","poliwrath");
  pokemons.insert("083","farfetchd");
  pokemons.insert("104","cubone");
  pokemons.insert("125","electabuzz");
  pokemons.insert("146","moltres");
  pokemons.insert("167","spinarak");
  pokemons.insert("188","skiploom");
  pokemons.insert("209","snubbull");
  pokemons.insert("230","kingdra");
  pokemons.insert("251","celebi");
  pokemons.insert("021","spearow");
  pokemons.insert("042","golbat");
  pokemons.insert("063","abra");
  pokemons.insert("084","doduo");
  pokemons.insert("105","marowak");
  pokemons.insert("126","magmar");
  pokemons.insert("147","dratini");
  pokemons.insert("168","ariados");
  pokemons.insert("189","jumpluff");
  pokemons.insert("210","granbull");
  pokemons.insert("231","phanpy");

  let pknb = match env::var("POKEMON_NUMBER") {
  Ok(val) => u32::from_str(&*val).unwrap(),
  Err(_) => 25,
};
let pknbstring = format!("{:03}", pknb);
return pokemons.get(&*pknbstring).map(|p| Pokemon { number:pknb, number_string:pknbstring, name:p.to_string() }).unwrap_or(Pokemon { number:25, number_string:"025".to_string(), name:"pikachu".to_string()})


}

fn message()->String { match env::var("MESSAGE") {
  Ok(val) => val,
  Err(_) => "no $MESSAGE env variable".to_string(),
}
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}


fn main() {

  let port = match env::var("PORT") {
    Ok(val) => val,
    Err(_) => "8080".to_string(),
  };

  let assets_dir_path = match env::var("ASSETS_DIR") {
    Ok(val) => val,
    Err(_) => "assets/".to_string(),
  };





  println!("Starting to run as pokemon {} #{} (configure using POKEMON_NUMBER)",pokemon().name, pokemon().number_string);
  println!("Found messgae {} (configure using MESSAGE)",message());
  println!("using assets path {} (configure using ASSETS_DIR)",assets_dir_path);




    fn hello_world(req: &mut Request) -> IronResult<Response> {

      let name_capitalize = some_kind_of_uppercase_first_letter(&*(pokemon().name.clone()));
      let imgpath = format!("/assets/SVG/{}-{}.svg", pokemon().number_string, pokemon().name);

      let page = format!(r##"<html>
      <head>
        <title>{} - Pokemon</title>
      </head>
      <body style="text-align:center">
        <H1>{}</H1>
        <p>{}</p>
        <img src="{}" style="width:500px;max-width:90%;height:auto;max-height:80%" />
      </body>
      </html>"##, name_capitalize, name_capitalize, message(), imgpath);

            Ok(Response::with((ContentType::html().0,status::Ok,  format!("{}", page))))
    }




let mut mount = Mount::new();
mount.mount("/", hello_world);
mount.mount("/assets", Static::new(Path::new(&*assets_dir_path)));

    let listen_target = "0.0.0.0:".to_string() + &*port;
    let _server = Iron::new(mount).http(&*listen_target).unwrap();
    println!("Listen on {} (configure using PORT)",listen_target);
}
