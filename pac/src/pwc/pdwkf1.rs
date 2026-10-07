#[doc = "Register `PDWKF1` reader"]
pub type R = crate::R<Pdwkf1Spec>;
#[doc = "Register `PDWKF1` writer"]
pub type W = crate::W<Pdwkf1Spec>;
#[doc = "Field `RTCPRDWKF` reader - desc RTCPRDWKF"]
pub type RtcprdwkfR = crate::BitReader;
#[doc = "Field `RTCPRDWKF` writer - desc RTCPRDWKF"]
pub type RtcprdwkfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCALMWKF` reader - desc RTCALMWKF"]
pub type RtcalmwkfR = crate::BitReader;
#[doc = "Field `RTCALMWKF` writer - desc RTCALMWKF"]
pub type RtcalmwkfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKTMWKF` reader - desc WKTMWKF"]
pub type WktmwkfR = crate::BitReader;
#[doc = "Field `WKTMWKF` writer - desc WKTMWKF"]
pub type WktmwkfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - desc RTCPRDWKF"]
    #[inline(always)]
    pub fn rtcprdwkf(&self) -> RtcprdwkfR {
        RtcprdwkfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RTCALMWKF"]
    #[inline(always)]
    pub fn rtcalmwkf(&self) -> RtcalmwkfR {
        RtcalmwkfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc WKTMWKF"]
    #[inline(always)]
    pub fn wktmwkf(&self) -> WktmwkfR {
        WktmwkfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - desc RTCPRDWKF"]
    #[inline(always)]
    pub fn rtcprdwkf(&mut self) -> RtcprdwkfW<'_, Pdwkf1Spec> {
        RtcprdwkfW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RTCALMWKF"]
    #[inline(always)]
    pub fn rtcalmwkf(&mut self) -> RtcalmwkfW<'_, Pdwkf1Spec> {
        RtcalmwkfW::new(self, 5)
    }
    #[doc = "Bit 7 - desc WKTMWKF"]
    #[inline(always)]
    pub fn wktmwkf(&mut self) -> WktmwkfW<'_, Pdwkf1Spec> {
        WktmwkfW::new(self, 7)
    }
}
#[doc = "desc PDWKF1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwkf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwkf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdwkf1Spec;
impl crate::RegisterSpec for Pdwkf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdwkf1::R`](R) reader structure"]
impl crate::Readable for Pdwkf1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdwkf1::W`](W) writer structure"]
impl crate::Writable for Pdwkf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDWKF1 to value 0"]
impl crate::Resettable for Pdwkf1Spec {}
