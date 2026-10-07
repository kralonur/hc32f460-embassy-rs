#[doc = "Register `ECONR` reader"]
pub type R = crate::R<EconrSpec>;
#[doc = "Register `ECONR` writer"]
pub type W = crate::W<EconrSpec>;
#[doc = "Field `ETEN1` reader - desc ETEN1"]
pub type Eten1R = crate::BitReader;
#[doc = "Field `ETEN1` writer - desc ETEN1"]
pub type Eten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN2` reader - desc ETEN2"]
pub type Eten2R = crate::BitReader;
#[doc = "Field `ETEN2` writer - desc ETEN2"]
pub type Eten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN3` reader - desc ETEN3"]
pub type Eten3R = crate::BitReader;
#[doc = "Field `ETEN3` writer - desc ETEN3"]
pub type Eten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN4` reader - desc ETEN4"]
pub type Eten4R = crate::BitReader;
#[doc = "Field `ETEN4` writer - desc ETEN4"]
pub type Eten4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN5` reader - desc ETEN5"]
pub type Eten5R = crate::BitReader;
#[doc = "Field `ETEN5` writer - desc ETEN5"]
pub type Eten5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN6` reader - desc ETEN6"]
pub type Eten6R = crate::BitReader;
#[doc = "Field `ETEN6` writer - desc ETEN6"]
pub type Eten6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN7` reader - desc ETEN7"]
pub type Eten7R = crate::BitReader;
#[doc = "Field `ETEN7` writer - desc ETEN7"]
pub type Eten7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN8` reader - desc ETEN8"]
pub type Eten8R = crate::BitReader;
#[doc = "Field `ETEN8` writer - desc ETEN8"]
pub type Eten8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ETEN1"]
    #[inline(always)]
    pub fn eten1(&self) -> Eten1R {
        Eten1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc ETEN2"]
    #[inline(always)]
    pub fn eten2(&self) -> Eten2R {
        Eten2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ETEN3"]
    #[inline(always)]
    pub fn eten3(&self) -> Eten3R {
        Eten3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ETEN4"]
    #[inline(always)]
    pub fn eten4(&self) -> Eten4R {
        Eten4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ETEN5"]
    #[inline(always)]
    pub fn eten5(&self) -> Eten5R {
        Eten5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ETEN6"]
    #[inline(always)]
    pub fn eten6(&self) -> Eten6R {
        Eten6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ETEN7"]
    #[inline(always)]
    pub fn eten7(&self) -> Eten7R {
        Eten7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ETEN8"]
    #[inline(always)]
    pub fn eten8(&self) -> Eten8R {
        Eten8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ETEN1"]
    #[inline(always)]
    pub fn eten1(&mut self) -> Eten1W<'_, EconrSpec> {
        Eten1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc ETEN2"]
    #[inline(always)]
    pub fn eten2(&mut self) -> Eten2W<'_, EconrSpec> {
        Eten2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc ETEN3"]
    #[inline(always)]
    pub fn eten3(&mut self) -> Eten3W<'_, EconrSpec> {
        Eten3W::new(self, 2)
    }
    #[doc = "Bit 3 - desc ETEN4"]
    #[inline(always)]
    pub fn eten4(&mut self) -> Eten4W<'_, EconrSpec> {
        Eten4W::new(self, 3)
    }
    #[doc = "Bit 4 - desc ETEN5"]
    #[inline(always)]
    pub fn eten5(&mut self) -> Eten5W<'_, EconrSpec> {
        Eten5W::new(self, 4)
    }
    #[doc = "Bit 5 - desc ETEN6"]
    #[inline(always)]
    pub fn eten6(&mut self) -> Eten6W<'_, EconrSpec> {
        Eten6W::new(self, 5)
    }
    #[doc = "Bit 6 - desc ETEN7"]
    #[inline(always)]
    pub fn eten7(&mut self) -> Eten7W<'_, EconrSpec> {
        Eten7W::new(self, 6)
    }
    #[doc = "Bit 7 - desc ETEN8"]
    #[inline(always)]
    pub fn eten8(&mut self) -> Eten8W<'_, EconrSpec> {
        Eten8W::new(self, 7)
    }
}
#[doc = "desc ECONR\n\nYou can [`read`](crate::Reg::read) this register and get [`econr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`econr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EconrSpec;
impl crate::RegisterSpec for EconrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`econr::R`](R) reader structure"]
impl crate::Readable for EconrSpec {}
#[doc = "`write(|w| ..)` method takes [`econr::W`](W) writer structure"]
impl crate::Writable for EconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECONR to value 0"]
impl crate::Resettable for EconrSpec {}
