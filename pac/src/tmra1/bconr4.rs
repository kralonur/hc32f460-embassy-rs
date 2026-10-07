#[doc = "Register `BCONR4` reader"]
pub type R = crate::R<Bconr4Spec>;
#[doc = "Register `BCONR4` writer"]
pub type W = crate::W<Bconr4Spec>;
#[doc = "Field `BEN` reader - desc BEN"]
pub type BenR = crate::BitReader;
#[doc = "Field `BEN` writer - desc BEN"]
pub type BenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSE0` reader - desc BSE0"]
pub type Bse0R = crate::BitReader;
#[doc = "Field `BSE0` writer - desc BSE0"]
pub type Bse0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSE1` reader - desc BSE1"]
pub type Bse1R = crate::BitReader;
#[doc = "Field `BSE1` writer - desc BSE1"]
pub type Bse1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BEN"]
    #[inline(always)]
    pub fn ben(&self) -> BenR {
        BenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BSE0"]
    #[inline(always)]
    pub fn bse0(&self) -> Bse0R {
        Bse0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BSE1"]
    #[inline(always)]
    pub fn bse1(&self) -> Bse1R {
        Bse1R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BEN"]
    #[inline(always)]
    pub fn ben(&mut self) -> BenW<'_, Bconr4Spec> {
        BenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BSE0"]
    #[inline(always)]
    pub fn bse0(&mut self) -> Bse0W<'_, Bconr4Spec> {
        Bse0W::new(self, 1)
    }
    #[doc = "Bit 2 - desc BSE1"]
    #[inline(always)]
    pub fn bse1(&mut self) -> Bse1W<'_, Bconr4Spec> {
        Bse1W::new(self, 2)
    }
}
#[doc = "desc BCONR4\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bconr4Spec;
impl crate::RegisterSpec for Bconr4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bconr4::R`](R) reader structure"]
impl crate::Readable for Bconr4Spec {}
#[doc = "`write(|w| ..)` method takes [`bconr4::W`](W) writer structure"]
impl crate::Writable for Bconr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCONR4 to value 0"]
impl crate::Resettable for Bconr4Spec {}
