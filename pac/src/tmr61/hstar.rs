#[doc = "Register `HSTAR` reader"]
pub type R = crate::R<HstarSpec>;
#[doc = "Register `HSTAR` writer"]
pub type W = crate::W<HstarSpec>;
#[doc = "Field `HSTA0` reader - desc HSTA0"]
pub type Hsta0R = crate::BitReader;
#[doc = "Field `HSTA0` writer - desc HSTA0"]
pub type Hsta0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA1` reader - desc HSTA1"]
pub type Hsta1R = crate::BitReader;
#[doc = "Field `HSTA1` writer - desc HSTA1"]
pub type Hsta1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA4` reader - desc HSTA4"]
pub type Hsta4R = crate::BitReader;
#[doc = "Field `HSTA4` writer - desc HSTA4"]
pub type Hsta4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA5` reader - desc HSTA5"]
pub type Hsta5R = crate::BitReader;
#[doc = "Field `HSTA5` writer - desc HSTA5"]
pub type Hsta5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA6` reader - desc HSTA6"]
pub type Hsta6R = crate::BitReader;
#[doc = "Field `HSTA6` writer - desc HSTA6"]
pub type Hsta6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA7` reader - desc HSTA7"]
pub type Hsta7R = crate::BitReader;
#[doc = "Field `HSTA7` writer - desc HSTA7"]
pub type Hsta7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA8` reader - desc HSTA8"]
pub type Hsta8R = crate::BitReader;
#[doc = "Field `HSTA8` writer - desc HSTA8"]
pub type Hsta8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA9` reader - desc HSTA9"]
pub type Hsta9R = crate::BitReader;
#[doc = "Field `HSTA9` writer - desc HSTA9"]
pub type Hsta9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA10` reader - desc HSTA10"]
pub type Hsta10R = crate::BitReader;
#[doc = "Field `HSTA10` writer - desc HSTA10"]
pub type Hsta10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTA11` reader - desc HSTA11"]
pub type Hsta11R = crate::BitReader;
#[doc = "Field `HSTA11` writer - desc HSTA11"]
pub type Hsta11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAS` reader - desc STAS"]
pub type StasR = crate::BitReader;
#[doc = "Field `STAS` writer - desc STAS"]
pub type StasW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - desc HSTA4"]
    #[inline(always)]
    pub fn hsta4(&self) -> Hsta4R {
        Hsta4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HSTA5"]
    #[inline(always)]
    pub fn hsta5(&self) -> Hsta5R {
        Hsta5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSTA6"]
    #[inline(always)]
    pub fn hsta6(&self) -> Hsta6R {
        Hsta6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HSTA7"]
    #[inline(always)]
    pub fn hsta7(&self) -> Hsta7R {
        Hsta7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSTA8"]
    #[inline(always)]
    pub fn hsta8(&self) -> Hsta8R {
        Hsta8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HSTA9"]
    #[inline(always)]
    pub fn hsta9(&self) -> Hsta9R {
        Hsta9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HSTA10"]
    #[inline(always)]
    pub fn hsta10(&self) -> Hsta10R {
        Hsta10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSTA11"]
    #[inline(always)]
    pub fn hsta11(&self) -> Hsta11R {
        Hsta11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - desc STAS"]
    #[inline(always)]
    pub fn stas(&self) -> StasR {
        StasR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HSTA0"]
    #[inline(always)]
    pub fn hsta0(&mut self) -> Hsta0W<'_, HstarSpec> {
        Hsta0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HSTA1"]
    #[inline(always)]
    pub fn hsta1(&mut self) -> Hsta1W<'_, HstarSpec> {
        Hsta1W::new(self, 1)
    }
    #[doc = "Bit 4 - desc HSTA4"]
    #[inline(always)]
    pub fn hsta4(&mut self) -> Hsta4W<'_, HstarSpec> {
        Hsta4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HSTA5"]
    #[inline(always)]
    pub fn hsta5(&mut self) -> Hsta5W<'_, HstarSpec> {
        Hsta5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HSTA6"]
    #[inline(always)]
    pub fn hsta6(&mut self) -> Hsta6W<'_, HstarSpec> {
        Hsta6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HSTA7"]
    #[inline(always)]
    pub fn hsta7(&mut self) -> Hsta7W<'_, HstarSpec> {
        Hsta7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HSTA8"]
    #[inline(always)]
    pub fn hsta8(&mut self) -> Hsta8W<'_, HstarSpec> {
        Hsta8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HSTA9"]
    #[inline(always)]
    pub fn hsta9(&mut self) -> Hsta9W<'_, HstarSpec> {
        Hsta9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HSTA10"]
    #[inline(always)]
    pub fn hsta10(&mut self) -> Hsta10W<'_, HstarSpec> {
        Hsta10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc HSTA11"]
    #[inline(always)]
    pub fn hsta11(&mut self) -> Hsta11W<'_, HstarSpec> {
        Hsta11W::new(self, 11)
    }
    #[doc = "Bit 31 - desc STAS"]
    #[inline(always)]
    pub fn stas(&mut self) -> StasW<'_, HstarSpec> {
        StasW::new(self, 31)
    }
}
#[doc = "desc HSTAR\n\nYou can [`read`](crate::Reg::read) this register and get [`hstar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hstar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HstarSpec;
impl crate::RegisterSpec for HstarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstar::R`](R) reader structure"]
impl crate::Readable for HstarSpec {}
#[doc = "`write(|w| ..)` method takes [`hstar::W`](W) writer structure"]
impl crate::Writable for HstarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSTAR to value 0"]
impl crate::Resettable for HstarSpec {}
