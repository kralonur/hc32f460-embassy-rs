#[doc = "Register `DOEPTSIZ0` reader"]
pub type R = crate::R<Doeptsiz0Spec>;
#[doc = "Register `DOEPTSIZ0` writer"]
pub type W = crate::W<Doeptsiz0Spec>;
#[doc = "Field `XFRSIZ` reader - desc XFRSIZ"]
pub type XfrsizR = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - desc XFRSIZ"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - desc PKTCNT"]
pub type PktcntR = crate::BitReader;
#[doc = "Field `PKTCNT` writer - desc PKTCNT"]
pub type PktcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPCNT` reader - desc STUPCNT"]
pub type StupcntR = crate::FieldReader;
#[doc = "Field `STUPCNT` writer - desc STUPCNT"]
pub type StupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - desc XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XfrsizR {
        XfrsizR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - desc PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - desc STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&self) -> StupcntR {
        StupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<'_, Doeptsiz0Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bit 19 - desc PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<'_, Doeptsiz0Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - desc STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&mut self) -> StupcntW<'_, Doeptsiz0Spec> {
        StupcntW::new(self, 29)
    }
}
#[doc = "desc DOEPTSIZ0\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doeptsiz0Spec;
impl crate::RegisterSpec for Doeptsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz0::R`](R) reader structure"]
impl crate::Readable for Doeptsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure"]
impl crate::Writable for Doeptsiz0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for Doeptsiz0Spec {}
