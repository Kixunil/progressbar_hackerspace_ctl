use ::reqwest::Client;

pub struct Lights {
    client: Client,
}

impl Lights {
    pub fn new() -> ::reqwest::Result<Self> {
        Client::new().map(|client| Lights { client })
    }

    pub fn main_room<'a>(&'a mut self) -> MainRoom<'a> {
        MainRoom { client: &mut self.client }
    }

    pub fn lab<'a>(&'a mut self) -> Lab<'a> {
        Lab { client: &mut self.client }
    }
}

pub struct MainRoom<'a> {
    client: &'a mut Client,
}

impl<'a> MainRoom<'a> {
    pub fn front_light<'b>(&'b mut self) -> Light<'b> {
        Light { client: self.client, url: "main/front" }
    }

    pub fn back_light<'b>(&'b mut self) -> Light<'b> {
        Light { client: self.client, url: "main/back" }
    }
}

pub struct Lab<'a> {
    client: &'a mut Client,
}

impl<'a> Lab<'a> {
    pub fn left_light<'b>(&'b mut self) -> Light<'b> {
        Light { client: self.client, url: "lab/left" }
    }

    pub fn right_light<'b>(&'b mut self) -> Light<'b> {
        Light { client: self.client, url: "lab/right" }
    }
}

pub struct Light<'a> {
    client: &'a mut Client,
    url: &'static str,
}

impl<'a> Light<'a> {
    pub fn turn_on(&mut self) {
        self.client
            .get(&format!("http://192.168.223.24/lights/{}/On", self.url))
            .send()
            .unwrap();
    }

    pub fn turn_off(&mut self) {
        self.client
            .get(&format!("http://192.168.223.24/lights/{}/Off", self.url))
            .send()
            .unwrap();
    }
}

pub trait EnumerateLights {
    fn enumerate_lights<F>(&mut self, f: F) where for<'a> F: FnMut(Light<'a>);
}

impl<'a> EnumerateLights for MainRoom<'a> {
    fn enumerate_lights<F>(&mut self, mut f: F) where for<'b> F: FnMut(Light<'b>) {
        f(self.front_light());
        f(self.back_light());
    }
}

impl<'a> EnumerateLights for Lab<'a> {
    fn enumerate_lights<F>(&mut self, mut f: F) where for<'b> F: FnMut(Light<'b>) {
        f(self.left_light());
        f(self.right_light());
    }
}

impl EnumerateLights for Lights {
    fn enumerate_lights<F>(&mut self, mut f: F) where for<'a> F: FnMut(Light<'a>) {
	self.main_room().enumerate_lights(&mut f);
	self.lab().enumerate_lights(&mut f);
    }
}
