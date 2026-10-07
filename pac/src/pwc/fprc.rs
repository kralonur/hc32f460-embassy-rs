#[doc = "Register `FPRC` reader"]
pub type R = crate::R<FprcSpec>;
#[doc = "Register `FPRC` writer"]
pub type W = crate::W<FprcSpec>;
#[doc = "Field `FPRCB0` reader - desc FPRCB0"]
pub type Fprcb0R = crate::BitReader;
#[doc = "Field `FPRCB0` writer - desc FPRCB0"]
pub type Fprcb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPRCB1` reader - desc FPRCB1"]
pub type Fprcb1R = crate::BitReader;
#[doc = "Field `FPRCB1` writer - desc FPRCB1"]
pub type Fprcb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPRCB2` reader - desc FPRCB2"]
pub type Fprcb2R = crate::BitReader;
#[doc = "Field `FPRCB2` writer - desc FPRCB2"]
pub type Fprcb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPRCB3` reader - desc FPRCB3"]
pub type Fprcb3R = crate::BitReader;
#[doc = "Field `FPRCB3` writer - desc FPRCB3"]
pub type Fprcb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPRCWE` reader - desc FPRCWE"]
pub type FprcweR = crate::FieldReader;
#[doc = "Field `FPRCWE` writer - desc FPRCWE"]
pub type FprcweW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - desc FPRCB0"]
    #[inline(always)]
    pub fn fprcb0(&self) -> Fprcb0R {
        Fprcb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FPRCB1"]
    #[inline(always)]
    pub fn fprcb1(&self) -> Fprcb1R {
        Fprcb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc FPRCB2"]
    #[inline(always)]
    pub fn fprcb2(&self) -> Fprcb2R {
        Fprcb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc FPRCB3"]
    #[inline(always)]
    pub fn fprcb3(&self) -> Fprcb3R {
        Fprcb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc FPRCWE"]
    #[inline(always)]
    pub fn fprcwe(&self) -> FprcweR {
        FprcweR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc FPRCB0"]
    #[inline(always)]
    pub fn fprcb0(&mut self) -> Fprcb0W<'_, FprcSpec> {
        Fprcb0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc FPRCB1"]
    #[inline(always)]
    pub fn fprcb1(&mut self) -> Fprcb1W<'_, FprcSpec> {
        Fprcb1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc FPRCB2"]
    #[inline(always)]
    pub fn fprcb2(&mut self) -> Fprcb2W<'_, FprcSpec> {
        Fprcb2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc FPRCB3"]
    #[inline(always)]
    pub fn fprcb3(&mut self) -> Fprcb3W<'_, FprcSpec> {
        Fprcb3W::new(self, 3)
    }
    #[doc = "Bits 8:15 - desc FPRCWE"]
    #[inline(always)]
    pub fn fprcwe(&mut self) -> FprcweW<'_, FprcSpec> {
        FprcweW::new(self, 8)
    }
}
#[doc = "desc FPRC\n\nYou can [`read`](crate::Reg::read) this register and get [`fprc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fprc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FprcSpec;
impl crate::RegisterSpec for FprcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fprc::R`](R) reader structure"]
impl crate::Readable for FprcSpec {}
#[doc = "`write(|w| ..)` method takes [`fprc::W`](W) writer structure"]
impl crate::Writable for FprcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPRC to value 0"]
impl crate::Resettable for FprcSpec {}
