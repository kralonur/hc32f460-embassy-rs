#[doc = "Register `DOEPTSIZ5` reader"]
pub type R = crate::R<Doeptsiz5Spec>;
#[doc = "Register `DOEPTSIZ5` writer"]
pub type W = crate::W<Doeptsiz5Spec>;
#[doc = "Field `XFRSIZ` reader - desc XFRSIZ"]
pub type XfrsizR = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - desc XFRSIZ"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - desc PKTCNT"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - desc PKTCNT"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
}
impl W {
    #[doc = "Bits 0:18 - desc XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<'_, Doeptsiz5Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:28 - desc PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Doeptsiz5Spec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "desc DOEPTSIZ5\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doeptsiz5Spec;
impl crate::RegisterSpec for Doeptsiz5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz5::R`](R) reader structure"]
impl crate::Readable for Doeptsiz5Spec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz5::W`](W) writer structure"]
impl crate::Writable for Doeptsiz5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPTSIZ5 to value 0"]
impl crate::Resettable for Doeptsiz5Spec {}
