#[doc = "Register `HCDOR` reader"]
pub type R = crate::R<HcdorSpec>;
#[doc = "Register `HCDOR` writer"]
pub type W = crate::W<HcdorSpec>;
#[doc = "Field `HCDO0` reader - desc HCDO0"]
pub type Hcdo0R = crate::BitReader;
#[doc = "Field `HCDO0` writer - desc HCDO0"]
pub type Hcdo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO1` reader - desc HCDO1"]
pub type Hcdo1R = crate::BitReader;
#[doc = "Field `HCDO1` writer - desc HCDO1"]
pub type Hcdo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO2` reader - desc HCDO2"]
pub type Hcdo2R = crate::BitReader;
#[doc = "Field `HCDO2` writer - desc HCDO2"]
pub type Hcdo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO3` reader - desc HCDO3"]
pub type Hcdo3R = crate::BitReader;
#[doc = "Field `HCDO3` writer - desc HCDO3"]
pub type Hcdo3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO4` reader - desc HCDO4"]
pub type Hcdo4R = crate::BitReader;
#[doc = "Field `HCDO4` writer - desc HCDO4"]
pub type Hcdo4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO5` reader - desc HCDO5"]
pub type Hcdo5R = crate::BitReader;
#[doc = "Field `HCDO5` writer - desc HCDO5"]
pub type Hcdo5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO6` reader - desc HCDO6"]
pub type Hcdo6R = crate::BitReader;
#[doc = "Field `HCDO6` writer - desc HCDO6"]
pub type Hcdo6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO7` reader - desc HCDO7"]
pub type Hcdo7R = crate::BitReader;
#[doc = "Field `HCDO7` writer - desc HCDO7"]
pub type Hcdo7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO8` reader - desc HCDO8"]
pub type Hcdo8R = crate::BitReader;
#[doc = "Field `HCDO8` writer - desc HCDO8"]
pub type Hcdo8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO9` reader - desc HCDO9"]
pub type Hcdo9R = crate::BitReader;
#[doc = "Field `HCDO9` writer - desc HCDO9"]
pub type Hcdo9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO10` reader - desc HCDO10"]
pub type Hcdo10R = crate::BitReader;
#[doc = "Field `HCDO10` writer - desc HCDO10"]
pub type Hcdo10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO11` reader - desc HCDO11"]
pub type Hcdo11R = crate::BitReader;
#[doc = "Field `HCDO11` writer - desc HCDO11"]
pub type Hcdo11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO16` reader - desc HCDO16"]
pub type Hcdo16R = crate::BitReader;
#[doc = "Field `HCDO16` writer - desc HCDO16"]
pub type Hcdo16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCDO17` reader - desc HCDO17"]
pub type Hcdo17R = crate::BitReader;
#[doc = "Field `HCDO17` writer - desc HCDO17"]
pub type Hcdo17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HCDO0"]
    #[inline(always)]
    pub fn hcdo0(&self) -> Hcdo0R {
        Hcdo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HCDO1"]
    #[inline(always)]
    pub fn hcdo1(&self) -> Hcdo1R {
        Hcdo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HCDO2"]
    #[inline(always)]
    pub fn hcdo2(&self) -> Hcdo2R {
        Hcdo2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc HCDO3"]
    #[inline(always)]
    pub fn hcdo3(&self) -> Hcdo3R {
        Hcdo3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc HCDO4"]
    #[inline(always)]
    pub fn hcdo4(&self) -> Hcdo4R {
        Hcdo4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc HCDO5"]
    #[inline(always)]
    pub fn hcdo5(&self) -> Hcdo5R {
        Hcdo5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HCDO6"]
    #[inline(always)]
    pub fn hcdo6(&self) -> Hcdo6R {
        Hcdo6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HCDO7"]
    #[inline(always)]
    pub fn hcdo7(&self) -> Hcdo7R {
        Hcdo7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HCDO8"]
    #[inline(always)]
    pub fn hcdo8(&self) -> Hcdo8R {
        Hcdo8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc HCDO9"]
    #[inline(always)]
    pub fn hcdo9(&self) -> Hcdo9R {
        Hcdo9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc HCDO10"]
    #[inline(always)]
    pub fn hcdo10(&self) -> Hcdo10R {
        Hcdo10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HCDO11"]
    #[inline(always)]
    pub fn hcdo11(&self) -> Hcdo11R {
        Hcdo11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - desc HCDO16"]
    #[inline(always)]
    pub fn hcdo16(&self) -> Hcdo16R {
        Hcdo16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc HCDO17"]
    #[inline(always)]
    pub fn hcdo17(&self) -> Hcdo17R {
        Hcdo17R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HCDO0"]
    #[inline(always)]
    pub fn hcdo0(&mut self) -> Hcdo0W<'_, HcdorSpec> {
        Hcdo0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc HCDO1"]
    #[inline(always)]
    pub fn hcdo1(&mut self) -> Hcdo1W<'_, HcdorSpec> {
        Hcdo1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc HCDO2"]
    #[inline(always)]
    pub fn hcdo2(&mut self) -> Hcdo2W<'_, HcdorSpec> {
        Hcdo2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc HCDO3"]
    #[inline(always)]
    pub fn hcdo3(&mut self) -> Hcdo3W<'_, HcdorSpec> {
        Hcdo3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc HCDO4"]
    #[inline(always)]
    pub fn hcdo4(&mut self) -> Hcdo4W<'_, HcdorSpec> {
        Hcdo4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc HCDO5"]
    #[inline(always)]
    pub fn hcdo5(&mut self) -> Hcdo5W<'_, HcdorSpec> {
        Hcdo5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc HCDO6"]
    #[inline(always)]
    pub fn hcdo6(&mut self) -> Hcdo6W<'_, HcdorSpec> {
        Hcdo6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc HCDO7"]
    #[inline(always)]
    pub fn hcdo7(&mut self) -> Hcdo7W<'_, HcdorSpec> {
        Hcdo7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc HCDO8"]
    #[inline(always)]
    pub fn hcdo8(&mut self) -> Hcdo8W<'_, HcdorSpec> {
        Hcdo8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc HCDO9"]
    #[inline(always)]
    pub fn hcdo9(&mut self) -> Hcdo9W<'_, HcdorSpec> {
        Hcdo9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc HCDO10"]
    #[inline(always)]
    pub fn hcdo10(&mut self) -> Hcdo10W<'_, HcdorSpec> {
        Hcdo10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc HCDO11"]
    #[inline(always)]
    pub fn hcdo11(&mut self) -> Hcdo11W<'_, HcdorSpec> {
        Hcdo11W::new(self, 11)
    }
    #[doc = "Bit 16 - desc HCDO16"]
    #[inline(always)]
    pub fn hcdo16(&mut self) -> Hcdo16W<'_, HcdorSpec> {
        Hcdo16W::new(self, 16)
    }
    #[doc = "Bit 17 - desc HCDO17"]
    #[inline(always)]
    pub fn hcdo17(&mut self) -> Hcdo17W<'_, HcdorSpec> {
        Hcdo17W::new(self, 17)
    }
}
#[doc = "desc HCDOR\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcdorSpec;
impl crate::RegisterSpec for HcdorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdor::R`](R) reader structure"]
impl crate::Readable for HcdorSpec {}
#[doc = "`write(|w| ..)` method takes [`hcdor::W`](W) writer structure"]
impl crate::Writable for HcdorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCDOR to value 0"]
impl crate::Resettable for HcdorSpec {}
