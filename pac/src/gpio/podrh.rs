#[doc = "Register `PODRH` reader"]
pub type R = crate::R<PodrhSpec>;
#[doc = "Register `PODRH` writer"]
pub type W = crate::W<PodrhSpec>;
#[doc = "Field `POUT00` reader - desc POUT00"]
pub type Pout00R = crate::BitReader;
#[doc = "Field `POUT00` writer - desc POUT00"]
pub type Pout00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT01` reader - desc POUT01"]
pub type Pout01R = crate::BitReader;
#[doc = "Field `POUT01` writer - desc POUT01"]
pub type Pout01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT02` reader - desc POUT02"]
pub type Pout02R = crate::BitReader;
#[doc = "Field `POUT02` writer - desc POUT02"]
pub type Pout02W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POUT00"]
    #[inline(always)]
    pub fn pout00(&self) -> Pout00R {
        Pout00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POUT01"]
    #[inline(always)]
    pub fn pout01(&self) -> Pout01R {
        Pout01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POUT02"]
    #[inline(always)]
    pub fn pout02(&self) -> Pout02R {
        Pout02R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POUT00"]
    #[inline(always)]
    pub fn pout00(&mut self) -> Pout00W<'_, PodrhSpec> {
        Pout00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POUT01"]
    #[inline(always)]
    pub fn pout01(&mut self) -> Pout01W<'_, PodrhSpec> {
        Pout01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POUT02"]
    #[inline(always)]
    pub fn pout02(&mut self) -> Pout02W<'_, PodrhSpec> {
        Pout02W::new(self, 2)
    }
}
#[doc = "desc PODRH\n\nYou can [`read`](crate::Reg::read) this register and get [`podrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PodrhSpec;
impl crate::RegisterSpec for PodrhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`podrh::R`](R) reader structure"]
impl crate::Readable for PodrhSpec {}
#[doc = "`write(|w| ..)` method takes [`podrh::W`](W) writer structure"]
impl crate::Writable for PodrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PODRH to value 0"]
impl crate::Resettable for PodrhSpec {}
