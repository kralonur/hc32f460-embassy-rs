#[doc = "Register `PEVNTNFCR` reader"]
pub type R = crate::R<PevntnfcrSpec>;
#[doc = "Register `PEVNTNFCR` writer"]
pub type W = crate::W<PevntnfcrSpec>;
#[doc = "Field `NFEN1` reader - desc NFEN1"]
pub type Nfen1R = crate::BitReader;
#[doc = "Field `NFEN1` writer - desc NFEN1"]
pub type Nfen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVS1` reader - desc DIVS1"]
pub type Divs1R = crate::FieldReader;
#[doc = "Field `DIVS1` writer - desc DIVS1"]
pub type Divs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN2` reader - desc NFEN2"]
pub type Nfen2R = crate::BitReader;
#[doc = "Field `NFEN2` writer - desc NFEN2"]
pub type Nfen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVS2` reader - desc DIVS2"]
pub type Divs2R = crate::FieldReader;
#[doc = "Field `DIVS2` writer - desc DIVS2"]
pub type Divs2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN3` reader - desc NFEN3"]
pub type Nfen3R = crate::BitReader;
#[doc = "Field `NFEN3` writer - desc NFEN3"]
pub type Nfen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVS3` reader - desc DIVS3"]
pub type Divs3R = crate::FieldReader;
#[doc = "Field `DIVS3` writer - desc DIVS3"]
pub type Divs3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN4` reader - desc NFEN4"]
pub type Nfen4R = crate::BitReader;
#[doc = "Field `NFEN4` writer - desc NFEN4"]
pub type Nfen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVS4` reader - desc DIVS4"]
pub type Divs4R = crate::FieldReader;
#[doc = "Field `DIVS4` writer - desc DIVS4"]
pub type Divs4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc NFEN1"]
    #[inline(always)]
    pub fn nfen1(&self) -> Nfen1R {
        Nfen1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc DIVS1"]
    #[inline(always)]
    pub fn divs1(&self) -> Divs1R {
        Divs1R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - desc NFEN2"]
    #[inline(always)]
    pub fn nfen2(&self) -> Nfen2R {
        Nfen2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - desc DIVS2"]
    #[inline(always)]
    pub fn divs2(&self) -> Divs2R {
        Divs2R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 16 - desc NFEN3"]
    #[inline(always)]
    pub fn nfen3(&self) -> Nfen3R {
        Nfen3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - desc DIVS3"]
    #[inline(always)]
    pub fn divs3(&self) -> Divs3R {
        Divs3R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 24 - desc NFEN4"]
    #[inline(always)]
    pub fn nfen4(&self) -> Nfen4R {
        Nfen4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - desc DIVS4"]
    #[inline(always)]
    pub fn divs4(&self) -> Divs4R {
        Divs4R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc NFEN1"]
    #[inline(always)]
    pub fn nfen1(&mut self) -> Nfen1W<'_, PevntnfcrSpec> {
        Nfen1W::new(self, 0)
    }
    #[doc = "Bits 1:2 - desc DIVS1"]
    #[inline(always)]
    pub fn divs1(&mut self) -> Divs1W<'_, PevntnfcrSpec> {
        Divs1W::new(self, 1)
    }
    #[doc = "Bit 8 - desc NFEN2"]
    #[inline(always)]
    pub fn nfen2(&mut self) -> Nfen2W<'_, PevntnfcrSpec> {
        Nfen2W::new(self, 8)
    }
    #[doc = "Bits 9:10 - desc DIVS2"]
    #[inline(always)]
    pub fn divs2(&mut self) -> Divs2W<'_, PevntnfcrSpec> {
        Divs2W::new(self, 9)
    }
    #[doc = "Bit 16 - desc NFEN3"]
    #[inline(always)]
    pub fn nfen3(&mut self) -> Nfen3W<'_, PevntnfcrSpec> {
        Nfen3W::new(self, 16)
    }
    #[doc = "Bits 17:18 - desc DIVS3"]
    #[inline(always)]
    pub fn divs3(&mut self) -> Divs3W<'_, PevntnfcrSpec> {
        Divs3W::new(self, 17)
    }
    #[doc = "Bit 24 - desc NFEN4"]
    #[inline(always)]
    pub fn nfen4(&mut self) -> Nfen4W<'_, PevntnfcrSpec> {
        Nfen4W::new(self, 24)
    }
    #[doc = "Bits 25:26 - desc DIVS4"]
    #[inline(always)]
    pub fn divs4(&mut self) -> Divs4W<'_, PevntnfcrSpec> {
        Divs4W::new(self, 25)
    }
}
#[doc = "desc PEVNTNFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntnfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntnfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PevntnfcrSpec;
impl crate::RegisterSpec for PevntnfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntnfcr::R`](R) reader structure"]
impl crate::Readable for PevntnfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pevntnfcr::W`](W) writer structure"]
impl crate::Writable for PevntnfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTNFCR to value 0"]
impl crate::Resettable for PevntnfcrSpec {}
