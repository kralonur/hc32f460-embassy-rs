#[doc = "Register `CHMUXR2` reader"]
pub type R = crate::R<Chmuxr2Spec>;
#[doc = "Register `CHMUXR2` writer"]
pub type W = crate::W<Chmuxr2Spec>;
#[doc = "Field `CH08MUX` reader - desc CH08MUX"]
pub type Ch08muxR = crate::FieldReader;
#[doc = "Field `CH08MUX` writer - desc CH08MUX"]
pub type Ch08muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH09MUX` reader - desc CH09MUX"]
pub type Ch09muxR = crate::FieldReader;
#[doc = "Field `CH09MUX` writer - desc CH09MUX"]
pub type Ch09muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH10MUX` reader - desc CH10MUX"]
pub type Ch10muxR = crate::FieldReader;
#[doc = "Field `CH10MUX` writer - desc CH10MUX"]
pub type Ch10muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH11MUX` reader - desc CH11MUX"]
pub type Ch11muxR = crate::FieldReader;
#[doc = "Field `CH11MUX` writer - desc CH11MUX"]
pub type Ch11muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CH08MUX"]
    #[inline(always)]
    pub fn ch08mux(&self) -> Ch08muxR {
        Ch08muxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc CH09MUX"]
    #[inline(always)]
    pub fn ch09mux(&self) -> Ch09muxR {
        Ch09muxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CH10MUX"]
    #[inline(always)]
    pub fn ch10mux(&self) -> Ch10muxR {
        Ch10muxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc CH11MUX"]
    #[inline(always)]
    pub fn ch11mux(&self) -> Ch11muxR {
        Ch11muxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CH08MUX"]
    #[inline(always)]
    pub fn ch08mux(&mut self) -> Ch08muxW<'_, Chmuxr2Spec> {
        Ch08muxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc CH09MUX"]
    #[inline(always)]
    pub fn ch09mux(&mut self) -> Ch09muxW<'_, Chmuxr2Spec> {
        Ch09muxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc CH10MUX"]
    #[inline(always)]
    pub fn ch10mux(&mut self) -> Ch10muxW<'_, Chmuxr2Spec> {
        Ch10muxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc CH11MUX"]
    #[inline(always)]
    pub fn ch11mux(&mut self) -> Ch11muxW<'_, Chmuxr2Spec> {
        Ch11muxW::new(self, 12)
    }
}
#[doc = "desc CHMUXR2\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmuxr2Spec;
impl crate::RegisterSpec for Chmuxr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chmuxr2::R`](R) reader structure"]
impl crate::Readable for Chmuxr2Spec {}
#[doc = "`write(|w| ..)` method takes [`chmuxr2::W`](W) writer structure"]
impl crate::Writable for Chmuxr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHMUXR2 to value 0xba98"]
impl crate::Resettable for Chmuxr2Spec {
    const RESET_VALUE: u16 = 0xba98;
}
