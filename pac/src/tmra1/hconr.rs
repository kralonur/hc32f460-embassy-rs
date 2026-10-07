#[doc = "Register `HCONR` reader"]
pub type R = crate::R<HconrSpec>;
#[doc = "Register `HCONR` writer"]
pub type W = crate::W<HconrSpec>;
#[doc = "Field `HSTA0` reader - desc HSTA0"]
pub type Hsta0R = crate::BitReader;
#[doc = "Field `HSTA0` writer - desc HSTA0"]
pub type Hsta0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA1` reader - desc HSTA1"]
pub type Hsta1R = crate::BitReader;
#[doc = "Field `HSTA1` writer - desc HSTA1"]
pub type Hsta1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA2` reader - desc HSTA2"]
pub type Hsta2R = crate::BitReader;
#[doc = "Field `HSTA2` writer - desc HSTA2"]
pub type Hsta2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP0` reader - desc HSTP0"]
pub type Hstp0R = crate::BitReader;
#[doc = "Field `HSTP0` writer - desc HSTP0"]
pub type Hstp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP1` reader - desc HSTP1"]
pub type Hstp1R = crate::BitReader;
#[doc = "Field `HSTP1` writer - desc HSTP1"]
pub type Hstp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTP2` reader - desc HSTP2"]
pub type Hstp2R = crate::BitReader;
#[doc = "Field `HSTP2` writer - desc HSTP2"]
pub type Hstp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE0` reader - desc HCLE0"]
pub type Hcle0R = crate::BitReader;
#[doc = "Field `HCLE0` writer - desc HCLE0"]
pub type Hcle0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE1` reader - desc HCLE1"]
pub type Hcle1R = crate::BitReader;
#[doc = "Field `HCLE1` writer - desc HCLE1"]
pub type Hcle1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE2` reader - desc HCLE2"]
pub type Hcle2R = crate::BitReader;
#[doc = "Field `HCLE2` writer - desc HCLE2"]
pub type Hcle2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE3` reader - desc HCLE3"]
pub type Hcle3R = crate::BitReader;
#[doc = "Field `HCLE3` writer - desc HCLE3"]
pub type Hcle3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE4` reader - desc HCLE4"]
pub type Hcle4R = crate::BitReader;
#[doc = "Field `HCLE4` writer - desc HCLE4"]
pub type Hcle4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE5` reader - desc HCLE5"]
pub type Hcle5R = crate::BitReader;
#[doc = "Field `HCLE5` writer - desc HCLE5"]
pub type Hcle5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE6` reader - desc HCLE6"]
pub type Hcle6R = crate::BitReader;
#[doc = "Field `HCLE6` writer - desc HCLE6"]
pub type Hcle6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&self) -> Hsta0R {
        Hsta0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&self) -> Hsta1R {
        Hsta1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSTA2"]
    #[inline(always)]
    pub fn hsta2(&self) -> Hsta2R {
        Hsta2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&self) -> Hstp0R {
        Hstp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&self) -> Hstp1R {
        Hstp1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSTP2"]
    #[inline(always)]
    pub fn hstp2(&self) -> Hstp2R {
        Hstp2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCLE0"]
    #[inline(always)]
    pub fn hcle0(&self) -> Hcle0R {
        Hcle0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCLE1"]
    #[inline(always)]
    pub fn hcle1(&self) -> Hcle1R {
        Hcle1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCLE2"]
    #[inline(always)]
    pub fn hcle2(&self) -> Hcle2R {
        Hcle2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HCLE3"]
    #[inline(always)]
    pub fn hcle3(&self) -> Hcle3R {
        Hcle3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc HCLE4"]
    #[inline(always)]
    pub fn hcle4(&self) -> Hcle4R {
        Hcle4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc HCLE5"]
    #[inline(always)]
    pub fn hcle5(&self) -> Hcle5R {
        Hcle5R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc HCLE6"]
    #[inline(always)]
    pub fn hcle6(&self) -> Hcle6R {
        Hcle6R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&mut self) -> Hsta0W<'_, HconrSpec> {
        Hsta0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&mut self) -> Hsta1W<'_, HconrSpec> {
        Hsta1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HSTA2"]
    #[inline(always)]
    pub fn hsta2(&mut self) -> Hsta2W<'_, HconrSpec> {
        Hsta2W::new(self, 2)
    }
    #[doc = "Bit 4 - desc HSTP0"]
    #[inline(always)]
    pub fn hstp0(&mut self) -> Hstp0W<'_, HconrSpec> {
        Hstp0W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HSTP1"]
    #[inline(always)]
    pub fn hstp1(&mut self) -> Hstp1W<'_, HconrSpec> {
        Hstp1W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HSTP2"]
    #[inline(always)]
    pub fn hstp2(&mut self) -> Hstp2W<'_, HconrSpec> {
        Hstp2W::new(self, 6)
    }
    #[doc = "Bit 8 - desc HCLE0"]
    #[inline(always)]
    pub fn hcle0(&mut self) -> Hcle0W<'_, HconrSpec> {
        Hcle0W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCLE1"]
    #[inline(always)]
    pub fn hcle1(&mut self) -> Hcle1W<'_, HconrSpec> {
        Hcle1W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HCLE2"]
    #[inline(always)]
    pub fn hcle2(&mut self) -> Hcle2W<'_, HconrSpec> {
        Hcle2W::new(self, 10)
    }
    #[doc = "Bit 12 - desc HCLE3"]
    #[inline(always)]
    pub fn hcle3(&mut self) -> Hcle3W<'_, HconrSpec> {
        Hcle3W::new(self, 12)
    }
    #[doc = "Bit 13 - desc HCLE4"]
    #[inline(always)]
    pub fn hcle4(&mut self) -> Hcle4W<'_, HconrSpec> {
        Hcle4W::new(self, 13)
    }
    #[doc = "Bit 14 - desc HCLE5"]
    #[inline(always)]
    pub fn hcle5(&mut self) -> Hcle5W<'_, HconrSpec> {
        Hcle5W::new(self, 14)
    }
    #[doc = "Bit 15 - desc HCLE6"]
    #[inline(always)]
    pub fn hcle6(&mut self) -> Hcle6W<'_, HconrSpec> {
        Hcle6W::new(self, 15)
    }
}
#[doc = "desc HCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`hconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HconrSpec;
impl crate::RegisterSpec for HconrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hconr::R`](R) reader structure"]
impl crate::Readable for HconrSpec {}
#[doc = "`write(|w| ..)` method takes [`hconr::W`](W) writer structure"]
impl crate::Writable for HconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCONR to value 0"]
impl crate::Resettable for HconrSpec {}
