#[doc = "Register `CHMUXR3` reader"]
pub type R = crate::R<Chmuxr3Spec>;
#[doc = "Register `CHMUXR3` writer"]
pub type W = crate::W<Chmuxr3Spec>;
#[doc = "Field `CH12MUX` reader - desc CH12MUX"]
pub type Ch12muxR = crate::FieldReader;
#[doc = "Field `CH12MUX` writer - desc CH12MUX"]
pub type Ch12muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH13MUX` reader - desc CH13MUX"]
pub type Ch13muxR = crate::FieldReader;
#[doc = "Field `CH13MUX` writer - desc CH13MUX"]
pub type Ch13muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH14MUX` reader - desc CH14MUX"]
pub type Ch14muxR = crate::FieldReader;
#[doc = "Field `CH14MUX` writer - desc CH14MUX"]
pub type Ch14muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH15MUX` reader - desc CH15MUX"]
pub type Ch15muxR = crate::FieldReader;
#[doc = "Field `CH15MUX` writer - desc CH15MUX"]
pub type Ch15muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CH12MUX"]
    #[inline(always)]
    pub fn ch12mux(&self) -> Ch12muxR {
        Ch12muxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc CH13MUX"]
    #[inline(always)]
    pub fn ch13mux(&self) -> Ch13muxR {
        Ch13muxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CH14MUX"]
    #[inline(always)]
    pub fn ch14mux(&self) -> Ch14muxR {
        Ch14muxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc CH15MUX"]
    #[inline(always)]
    pub fn ch15mux(&self) -> Ch15muxR {
        Ch15muxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CH12MUX"]
    #[inline(always)]
    pub fn ch12mux(&mut self) -> Ch12muxW<'_, Chmuxr3Spec> {
        Ch12muxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc CH13MUX"]
    #[inline(always)]
    pub fn ch13mux(&mut self) -> Ch13muxW<'_, Chmuxr3Spec> {
        Ch13muxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc CH14MUX"]
    #[inline(always)]
    pub fn ch14mux(&mut self) -> Ch14muxW<'_, Chmuxr3Spec> {
        Ch14muxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc CH15MUX"]
    #[inline(always)]
    pub fn ch15mux(&mut self) -> Ch15muxW<'_, Chmuxr3Spec> {
        Ch15muxW::new(self, 12)
    }
}
#[doc = "desc CHMUXR3\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmuxr3Spec;
impl crate::RegisterSpec for Chmuxr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chmuxr3::R`](R) reader structure"]
impl crate::Readable for Chmuxr3Spec {}
#[doc = "`write(|w| ..)` method takes [`chmuxr3::W`](W) writer structure"]
impl crate::Writable for Chmuxr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHMUXR3 to value 0xfedc"]
impl crate::Resettable for Chmuxr3Spec {
    const RESET_VALUE: u16 = 0xfedc;
}
