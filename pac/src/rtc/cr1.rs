#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `PRDS` reader - desc PRDS"]
pub type PrdsR = crate::FieldReader;
#[doc = "Field `PRDS` writer - desc PRDS"]
pub type PrdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AMPM` reader - desc AMPM"]
pub type AmpmR = crate::BitReader;
#[doc = "Field `AMPM` writer - desc AMPM"]
pub type AmpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMFCLR` reader - desc ALMFCLR"]
pub type AlmfclrR = crate::BitReader;
#[doc = "Field `ALMFCLR` writer - desc ALMFCLR"]
pub type AlmfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEHZOE` reader - desc ONEHZOE"]
pub type OnehzoeR = crate::BitReader;
#[doc = "Field `ONEHZOE` writer - desc ONEHZOE"]
pub type OnehzoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEHZSEL` reader - desc ONEHZSEL"]
pub type OnehzselR = crate::BitReader;
#[doc = "Field `ONEHZSEL` writer - desc ONEHZSEL"]
pub type OnehzselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc PRDS"]
    #[inline(always)]
    pub fn prds(&self) -> PrdsR {
        PrdsR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - desc AMPM"]
    #[inline(always)]
    pub fn ampm(&self) -> AmpmR {
        AmpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ALMFCLR"]
    #[inline(always)]
    pub fn almfclr(&self) -> AlmfclrR {
        AlmfclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ONEHZOE"]
    #[inline(always)]
    pub fn onehzoe(&self) -> OnehzoeR {
        OnehzoeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ONEHZSEL"]
    #[inline(always)]
    pub fn onehzsel(&self) -> OnehzselR {
        OnehzselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PRDS"]
    #[inline(always)]
    pub fn prds(&mut self) -> PrdsW<'_, Cr1Spec> {
        PrdsW::new(self, 0)
    }
    #[doc = "Bit 3 - desc AMPM"]
    #[inline(always)]
    pub fn ampm(&mut self) -> AmpmW<'_, Cr1Spec> {
        AmpmW::new(self, 3)
    }
    #[doc = "Bit 4 - desc ALMFCLR"]
    #[inline(always)]
    pub fn almfclr(&mut self) -> AlmfclrW<'_, Cr1Spec> {
        AlmfclrW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ONEHZOE"]
    #[inline(always)]
    pub fn onehzoe(&mut self) -> OnehzoeW<'_, Cr1Spec> {
        OnehzoeW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ONEHZSEL"]
    #[inline(always)]
    pub fn onehzsel(&mut self) -> OnehzselW<'_, Cr1Spec> {
        OnehzselW::new(self, 6)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr1Spec> {
        StartW::new(self, 7)
    }
}
#[doc = "desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
