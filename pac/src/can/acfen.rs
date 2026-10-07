#[doc = "Register `ACFEN` reader"]
pub type R = crate::R<AcfenSpec>;
#[doc = "Register `ACFEN` writer"]
pub type W = crate::W<AcfenSpec>;
#[doc = "Field `AE_1` reader - desc AE_1"]
pub type Ae1R = crate::BitReader;
#[doc = "Field `AE_1` writer - desc AE_1"]
pub type Ae1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_2` reader - desc AE_2"]
pub type Ae2R = crate::BitReader;
#[doc = "Field `AE_2` writer - desc AE_2"]
pub type Ae2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_3` reader - desc AE_3"]
pub type Ae3R = crate::BitReader;
#[doc = "Field `AE_3` writer - desc AE_3"]
pub type Ae3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_4` reader - desc AE_4"]
pub type Ae4R = crate::BitReader;
#[doc = "Field `AE_4` writer - desc AE_4"]
pub type Ae4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_5` reader - desc AE_5"]
pub type Ae5R = crate::BitReader;
#[doc = "Field `AE_5` writer - desc AE_5"]
pub type Ae5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_6` reader - desc AE_6"]
pub type Ae6R = crate::BitReader;
#[doc = "Field `AE_6` writer - desc AE_6"]
pub type Ae6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_7` reader - desc AE_7"]
pub type Ae7R = crate::BitReader;
#[doc = "Field `AE_7` writer - desc AE_7"]
pub type Ae7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_8` reader - desc AE_8"]
pub type Ae8R = crate::BitReader;
#[doc = "Field `AE_8` writer - desc AE_8"]
pub type Ae8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc AE_1"]
    #[inline(always)]
    pub fn ae_1(&self) -> Ae1R {
        Ae1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AE_2"]
    #[inline(always)]
    pub fn ae_2(&self) -> Ae2R {
        Ae2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AE_3"]
    #[inline(always)]
    pub fn ae_3(&self) -> Ae3R {
        Ae3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc AE_4"]
    #[inline(always)]
    pub fn ae_4(&self) -> Ae4R {
        Ae4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc AE_5"]
    #[inline(always)]
    pub fn ae_5(&self) -> Ae5R {
        Ae5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc AE_6"]
    #[inline(always)]
    pub fn ae_6(&self) -> Ae6R {
        Ae6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AE_7"]
    #[inline(always)]
    pub fn ae_7(&self) -> Ae7R {
        Ae7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AE_8"]
    #[inline(always)]
    pub fn ae_8(&self) -> Ae8R {
        Ae8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AE_1"]
    #[inline(always)]
    pub fn ae_1(&mut self) -> Ae1W<'_, AcfenSpec> {
        Ae1W::new(self, 0)
    }
    #[doc = "Bit 1 - desc AE_2"]
    #[inline(always)]
    pub fn ae_2(&mut self) -> Ae2W<'_, AcfenSpec> {
        Ae2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc AE_3"]
    #[inline(always)]
    pub fn ae_3(&mut self) -> Ae3W<'_, AcfenSpec> {
        Ae3W::new(self, 2)
    }
    #[doc = "Bit 3 - desc AE_4"]
    #[inline(always)]
    pub fn ae_4(&mut self) -> Ae4W<'_, AcfenSpec> {
        Ae4W::new(self, 3)
    }
    #[doc = "Bit 4 - desc AE_5"]
    #[inline(always)]
    pub fn ae_5(&mut self) -> Ae5W<'_, AcfenSpec> {
        Ae5W::new(self, 4)
    }
    #[doc = "Bit 5 - desc AE_6"]
    #[inline(always)]
    pub fn ae_6(&mut self) -> Ae6W<'_, AcfenSpec> {
        Ae6W::new(self, 5)
    }
    #[doc = "Bit 6 - desc AE_7"]
    #[inline(always)]
    pub fn ae_7(&mut self) -> Ae7W<'_, AcfenSpec> {
        Ae7W::new(self, 6)
    }
    #[doc = "Bit 7 - desc AE_8"]
    #[inline(always)]
    pub fn ae_8(&mut self) -> Ae8W<'_, AcfenSpec> {
        Ae8W::new(self, 7)
    }
}
#[doc = "desc ACFEN\n\nYou can [`read`](crate::Reg::read) this register and get [`acfen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acfen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcfenSpec;
impl crate::RegisterSpec for AcfenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`acfen::R`](R) reader structure"]
impl crate::Readable for AcfenSpec {}
#[doc = "`write(|w| ..)` method takes [`acfen::W`](W) writer structure"]
impl crate::Writable for AcfenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACFEN to value 0x01"]
impl crate::Resettable for AcfenSpec {
    const RESET_VALUE: u8 = 0x01;
}
