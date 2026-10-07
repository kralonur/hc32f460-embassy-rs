#[doc = "Register `HCTSIZ1` reader"]
pub type R = crate::R<Hctsiz1Spec>;
#[doc = "Register `HCTSIZ1` writer"]
pub type W = crate::W<Hctsiz1Spec>;
#[doc = "Field `XFRSIZ` reader - desc XFRSIZ"]
pub type XfrsizR = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - desc XFRSIZ"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - desc PKTCNT"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - desc PKTCNT"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - desc DPID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `DPID` writer - desc DPID"]
pub type DpidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - desc XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XfrsizR {
        XfrsizR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - desc PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - desc DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - desc XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<'_, Hctsiz1Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:28 - desc PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Hctsiz1Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - desc DPID"]
    #[inline(always)]
    pub fn dpid(&mut self) -> DpidW<'_, Hctsiz1Spec> {
        DpidW::new(self, 29)
    }
}
#[doc = "desc HCTSIZ1\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hctsiz1Spec;
impl crate::RegisterSpec for Hctsiz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz1::R`](R) reader structure"]
impl crate::Readable for Hctsiz1Spec {}
#[doc = "`write(|w| ..)` method takes [`hctsiz1::W`](W) writer structure"]
impl crate::Writable for Hctsiz1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCTSIZ1 to value 0"]
impl crate::Resettable for Hctsiz1Spec {}
