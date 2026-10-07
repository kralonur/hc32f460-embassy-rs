#[doc = "Register `DIEPTSIZ2` reader"]
pub type R = crate::R<Dieptsiz2Spec>;
#[doc = "Register `DIEPTSIZ2` writer"]
pub type W = crate::W<Dieptsiz2Spec>;
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
    pub fn xfrsiz(&mut self) -> XfrsizW<'_, Dieptsiz2Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:28 - desc PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Dieptsiz2Spec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "desc DIEPTSIZ2\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptsiz2Spec;
impl crate::RegisterSpec for Dieptsiz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz2::R`](R) reader structure"]
impl crate::Readable for Dieptsiz2Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz2::W`](W) writer structure"]
impl crate::Writable for Dieptsiz2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPTSIZ2 to value 0"]
impl crate::Resettable for Dieptsiz2Spec {}
