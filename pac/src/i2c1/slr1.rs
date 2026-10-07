#[doc = "Register `SLR1` reader"]
pub type R = crate::R<Slr1Spec>;
#[doc = "Register `SLR1` writer"]
pub type W = crate::W<Slr1Spec>;
#[doc = "Field `SLADDR1` reader - desc SLADDR1"]
pub type Sladdr1R = crate::FieldReader<u16>;
#[doc = "Field `SLADDR1` writer - desc SLADDR1"]
pub type Sladdr1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLADDR1EN` reader - desc SLADDR1EN"]
pub type Sladdr1enR = crate::BitReader;
#[doc = "Field `SLADDR1EN` writer - desc SLADDR1EN"]
pub type Sladdr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRMOD1` reader - desc ADDRMOD1"]
pub type Addrmod1R = crate::BitReader;
#[doc = "Field `ADDRMOD1` writer - desc ADDRMOD1"]
pub type Addrmod1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - desc SLADDR1"]
    #[inline(always)]
    pub fn sladdr1(&self) -> Sladdr1R {
        Sladdr1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - desc SLADDR1EN"]
    #[inline(always)]
    pub fn sladdr1en(&self) -> Sladdr1enR {
        Sladdr1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ADDRMOD1"]
    #[inline(always)]
    pub fn addrmod1(&self) -> Addrmod1R {
        Addrmod1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc SLADDR1"]
    #[inline(always)]
    pub fn sladdr1(&mut self) -> Sladdr1W<'_, Slr1Spec> {
        Sladdr1W::new(self, 0)
    }
    #[doc = "Bit 12 - desc SLADDR1EN"]
    #[inline(always)]
    pub fn sladdr1en(&mut self) -> Sladdr1enW<'_, Slr1Spec> {
        Sladdr1enW::new(self, 12)
    }
    #[doc = "Bit 15 - desc ADDRMOD1"]
    #[inline(always)]
    pub fn addrmod1(&mut self) -> Addrmod1W<'_, Slr1Spec> {
        Addrmod1W::new(self, 15)
    }
}
#[doc = "desc SLR1\n\nYou can [`read`](crate::Reg::read) this register and get [`slr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slr1Spec;
impl crate::RegisterSpec for Slr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slr1::R`](R) reader structure"]
impl crate::Readable for Slr1Spec {}
#[doc = "`write(|w| ..)` method takes [`slr1::W`](W) writer structure"]
impl crate::Writable for Slr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLR1 to value 0"]
impl crate::Resettable for Slr1Spec {}
