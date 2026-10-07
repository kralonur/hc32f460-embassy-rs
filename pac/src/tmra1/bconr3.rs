#[doc = "Register `BCONR3` reader"]
pub type R = crate::R<Bconr3Spec>;
#[doc = "Register `BCONR3` writer"]
pub type W = crate::W<Bconr3Spec>;
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
    pub fn ben(&mut self) -> BenW<'_, Bconr3Spec> {
        BenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc BSE0"]
    #[inline(always)]
    pub fn bse0(&mut self) -> Bse0W<'_, Bconr3Spec> {
        Bse0W::new(self, 1)
    }
    #[doc = "Bit 2 - desc BSE1"]
    #[inline(always)]
    pub fn bse1(&mut self) -> Bse1W<'_, Bconr3Spec> {
        Bse1W::new(self, 2)
    }
}
#[doc = "desc BCONR3\n\nYou can [`read`](crate::Reg::read) this register and get [`bconr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bconr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bconr3Spec;
impl crate::RegisterSpec for Bconr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bconr3::R`](R) reader structure"]
impl crate::Readable for Bconr3Spec {}
#[doc = "`write(|w| ..)` method takes [`bconr3::W`](W) writer structure"]
impl crate::Writable for Bconr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCONR3 to value 0"]
impl crate::Resettable for Bconr3Spec {}
