#[doc = "Register `HCLRR` reader"]
pub type R = crate::R<HclrrSpec>;
#[doc = "Register `HCLRR` writer"]
pub type W = crate::W<HclrrSpec>;
#[doc = "Field `HCLE0` reader - desc HCLE0"]
pub type Hcle0R = crate::BitReader;
#[doc = "Field `HCLE0` writer - desc HCLE0"]
pub type Hcle0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE1` reader - desc HCLE1"]
pub type Hcle1R = crate::BitReader;
#[doc = "Field `HCLE1` writer - desc HCLE1"]
pub type Hcle1W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `HCLE7` reader - desc HCLE7"]
pub type Hcle7R = crate::BitReader;
#[doc = "Field `HCLE7` writer - desc HCLE7"]
pub type Hcle7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE8` reader - desc HCLE8"]
pub type Hcle8R = crate::BitReader;
#[doc = "Field `HCLE8` writer - desc HCLE8"]
pub type Hcle8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE9` reader - desc HCLE9"]
pub type Hcle9R = crate::BitReader;
#[doc = "Field `HCLE9` writer - desc HCLE9"]
pub type Hcle9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE10` reader - desc HCLE10"]
pub type Hcle10R = crate::BitReader;
#[doc = "Field `HCLE10` writer - desc HCLE10"]
pub type Hcle10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLE11` reader - desc HCLE11"]
pub type Hcle11R = crate::BitReader;
#[doc = "Field `HCLE11` writer - desc HCLE11"]
pub type Hcle11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLES` reader - desc CLES"]
pub type ClesR = crate::BitReader;
#[doc = "Field `CLES` writer - desc CLES"]
pub type ClesW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCLE0"]
    #[inline(always)]
    pub fn hcle0(&self) -> Hcle0R {
        Hcle0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCLE1"]
    #[inline(always)]
    pub fn hcle1(&self) -> Hcle1R {
        Hcle1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCLE4"]
    #[inline(always)]
    pub fn hcle4(&self) -> Hcle4R {
        Hcle4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCLE5"]
    #[inline(always)]
    pub fn hcle5(&self) -> Hcle5R {
        Hcle5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCLE6"]
    #[inline(always)]
    pub fn hcle6(&self) -> Hcle6R {
        Hcle6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCLE7"]
    #[inline(always)]
    pub fn hcle7(&self) -> Hcle7R {
        Hcle7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCLE8"]
    #[inline(always)]
    pub fn hcle8(&self) -> Hcle8R {
        Hcle8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCLE9"]
    #[inline(always)]
    pub fn hcle9(&self) -> Hcle9R {
        Hcle9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCLE10"]
    #[inline(always)]
    pub fn hcle10(&self) -> Hcle10R {
        Hcle10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCLE11"]
    #[inline(always)]
    pub fn hcle11(&self) -> Hcle11R {
        Hcle11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - desc CLES"]
    #[inline(always)]
    pub fn cles(&self) -> ClesR {
        ClesR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCLE0"]
    #[inline(always)]
    pub fn hcle0(&mut self) -> Hcle0W<'_, HclrrSpec> {
        Hcle0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCLE1"]
    #[inline(always)]
    pub fn hcle1(&mut self) -> Hcle1W<'_, HclrrSpec> {
        Hcle1W::new(self, 1)
    }
    #[doc = "Bit 4 - desc HCLE4"]
    #[inline(always)]
    pub fn hcle4(&mut self) -> Hcle4W<'_, HclrrSpec> {
        Hcle4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HCLE5"]
    #[inline(always)]
    pub fn hcle5(&mut self) -> Hcle5W<'_, HclrrSpec> {
        Hcle5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HCLE6"]
    #[inline(always)]
    pub fn hcle6(&mut self) -> Hcle6W<'_, HclrrSpec> {
        Hcle6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HCLE7"]
    #[inline(always)]
    pub fn hcle7(&mut self) -> Hcle7W<'_, HclrrSpec> {
        Hcle7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HCLE8"]
    #[inline(always)]
    pub fn hcle8(&mut self) -> Hcle8W<'_, HclrrSpec> {
        Hcle8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCLE9"]
    #[inline(always)]
    pub fn hcle9(&mut self) -> Hcle9W<'_, HclrrSpec> {
        Hcle9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HCLE10"]
    #[inline(always)]
    pub fn hcle10(&mut self) -> Hcle10W<'_, HclrrSpec> {
        Hcle10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc HCLE11"]
    #[inline(always)]
    pub fn hcle11(&mut self) -> Hcle11W<'_, HclrrSpec> {
        Hcle11W::new(self, 11)
    }
    #[doc = "Bit 31 - desc CLES"]
    #[inline(always)]
    pub fn cles(&mut self) -> ClesW<'_, HclrrSpec> {
        ClesW::new(self, 31)
    }
}
#[doc = "desc HCLRR\n\nYou can [`read`](crate::Reg::read) this register and get [`hclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HclrrSpec;
impl crate::RegisterSpec for HclrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hclrr::R`](R) reader structure"]
impl crate::Readable for HclrrSpec {}
#[doc = "`write(|w| ..)` method takes [`hclrr::W`](W) writer structure"]
impl crate::Writable for HclrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCLRR to value 0"]
impl crate::Resettable for HclrrSpec {}
