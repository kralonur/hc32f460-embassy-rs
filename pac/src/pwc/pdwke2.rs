#[doc = "Register `PDWKE2` reader"]
pub type R = crate::R<Pdwke2Spec>;
#[doc = "Register `PDWKE2` writer"]
pub type W = crate::W<Pdwke2Spec>;
#[doc = "Field `VD1WKE` reader - desc VD1WKE"]
pub type Vd1wkeR = crate::BitReader;
#[doc = "Field `VD1WKE` writer - desc VD1WKE"]
pub type Vd1wkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VD2WKE` reader - desc VD2WKE"]
pub type Vd2wkeR = crate::BitReader;
#[doc = "Field `VD2WKE` writer - desc VD2WKE"]
pub type Vd2wkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMIWKE` reader - desc NMIWKE"]
pub type NmiwkeR = crate::BitReader;
#[doc = "Field `NMIWKE` writer - desc NMIWKE"]
pub type NmiwkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCPRDWKE` reader - desc RTCPRDWKE"]
pub type RtcprdwkeR = crate::BitReader;
#[doc = "Field `RTCPRDWKE` writer - desc RTCPRDWKE"]
pub type RtcprdwkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCALMWKE` reader - desc RTCALMWKE"]
pub type RtcalmwkeR = crate::BitReader;
#[doc = "Field `RTCALMWKE` writer - desc RTCALMWKE"]
pub type RtcalmwkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKTMWKE` reader - desc WKTMWKE"]
pub type WktmwkeR = crate::BitReader;
#[doc = "Field `WKTMWKE` writer - desc WKTMWKE"]
pub type WktmwkeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc VD1WKE"]
    #[inline(always)]
    pub fn vd1wke(&self) -> Vd1wkeR {
        Vd1wkeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc VD2WKE"]
    #[inline(always)]
    pub fn vd2wke(&self) -> Vd2wkeR {
        Vd2wkeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc NMIWKE"]
    #[inline(always)]
    pub fn nmiwke(&self) -> NmiwkeR {
        NmiwkeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RTCPRDWKE"]
    #[inline(always)]
    pub fn rtcprdwke(&self) -> RtcprdwkeR {
        RtcprdwkeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RTCALMWKE"]
    #[inline(always)]
    pub fn rtcalmwke(&self) -> RtcalmwkeR {
        RtcalmwkeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc WKTMWKE"]
    #[inline(always)]
    pub fn wktmwke(&self) -> WktmwkeR {
        WktmwkeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc VD1WKE"]
    #[inline(always)]
    pub fn vd1wke(&mut self) -> Vd1wkeW<'_, Pdwke2Spec> {
        Vd1wkeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc VD2WKE"]
    #[inline(always)]
    pub fn vd2wke(&mut self) -> Vd2wkeW<'_, Pdwke2Spec> {
        Vd2wkeW::new(self, 1)
    }
    #[doc = "Bit 2 - desc NMIWKE"]
    #[inline(always)]
    pub fn nmiwke(&mut self) -> NmiwkeW<'_, Pdwke2Spec> {
        NmiwkeW::new(self, 2)
    }
    #[doc = "Bit 4 - desc RTCPRDWKE"]
    #[inline(always)]
    pub fn rtcprdwke(&mut self) -> RtcprdwkeW<'_, Pdwke2Spec> {
        RtcprdwkeW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RTCALMWKE"]
    #[inline(always)]
    pub fn rtcalmwke(&mut self) -> RtcalmwkeW<'_, Pdwke2Spec> {
        RtcalmwkeW::new(self, 5)
    }
    #[doc = "Bit 7 - desc WKTMWKE"]
    #[inline(always)]
    pub fn wktmwke(&mut self) -> WktmwkeW<'_, Pdwke2Spec> {
        WktmwkeW::new(self, 7)
    }
}
#[doc = "desc PDWKE2\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwke2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwke2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdwke2Spec;
impl crate::RegisterSpec for Pdwke2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdwke2::R`](R) reader structure"]
impl crate::Readable for Pdwke2Spec {}
#[doc = "`write(|w| ..)` method takes [`pdwke2::W`](W) writer structure"]
impl crate::Writable for Pdwke2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDWKE2 to value 0"]
impl crate::Resettable for Pdwke2Spec {}
