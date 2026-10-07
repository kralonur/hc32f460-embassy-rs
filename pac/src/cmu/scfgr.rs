#[doc = "Register `SCFGR` reader"]
pub type R = crate::R<ScfgrSpec>;
#[doc = "Register `SCFGR` writer"]
pub type W = crate::W<ScfgrSpec>;
#[doc = "Field `PCLK0S` reader - desc PCLK0S"]
pub type Pclk0sR = crate::FieldReader;
#[doc = "Field `PCLK0S` writer - desc PCLK0S"]
pub type Pclk0sW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCLK1S` reader - desc PCLK1S"]
pub type Pclk1sR = crate::FieldReader;
#[doc = "Field `PCLK1S` writer - desc PCLK1S"]
pub type Pclk1sW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCLK2S` reader - desc PCLK2S"]
pub type Pclk2sR = crate::FieldReader;
#[doc = "Field `PCLK2S` writer - desc PCLK2S"]
pub type Pclk2sW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCLK3S` reader - desc PCLK3S"]
pub type Pclk3sR = crate::FieldReader;
#[doc = "Field `PCLK3S` writer - desc PCLK3S"]
pub type Pclk3sW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCLK4S` reader - desc PCLK4S"]
pub type Pclk4sR = crate::FieldReader;
#[doc = "Field `PCLK4S` writer - desc PCLK4S"]
pub type Pclk4sW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXCKS` reader - desc EXCKS"]
pub type ExcksR = crate::FieldReader;
#[doc = "Field `EXCKS` writer - desc EXCKS"]
pub type ExcksW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HCLKS` reader - desc HCLKS"]
pub type HclksR = crate::FieldReader;
#[doc = "Field `HCLKS` writer - desc HCLKS"]
pub type HclksW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc PCLK0S"]
    #[inline(always)]
    pub fn pclk0s(&self) -> Pclk0sR {
        Pclk0sR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc PCLK1S"]
    #[inline(always)]
    pub fn pclk1s(&self) -> Pclk1sR {
        Pclk1sR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc PCLK2S"]
    #[inline(always)]
    pub fn pclk2s(&self) -> Pclk2sR {
        Pclk2sR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc PCLK3S"]
    #[inline(always)]
    pub fn pclk3s(&self) -> Pclk3sR {
        Pclk3sR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - desc PCLK4S"]
    #[inline(always)]
    pub fn pclk4s(&self) -> Pclk4sR {
        Pclk4sR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - desc EXCKS"]
    #[inline(always)]
    pub fn excks(&self) -> ExcksR {
        ExcksR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - desc HCLKS"]
    #[inline(always)]
    pub fn hclks(&self) -> HclksR {
        HclksR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc PCLK0S"]
    #[inline(always)]
    pub fn pclk0s(&mut self) -> Pclk0sW<'_, ScfgrSpec> {
        Pclk0sW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc PCLK1S"]
    #[inline(always)]
    pub fn pclk1s(&mut self) -> Pclk1sW<'_, ScfgrSpec> {
        Pclk1sW::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc PCLK2S"]
    #[inline(always)]
    pub fn pclk2s(&mut self) -> Pclk2sW<'_, ScfgrSpec> {
        Pclk2sW::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc PCLK3S"]
    #[inline(always)]
    pub fn pclk3s(&mut self) -> Pclk3sW<'_, ScfgrSpec> {
        Pclk3sW::new(self, 12)
    }
    #[doc = "Bits 16:18 - desc PCLK4S"]
    #[inline(always)]
    pub fn pclk4s(&mut self) -> Pclk4sW<'_, ScfgrSpec> {
        Pclk4sW::new(self, 16)
    }
    #[doc = "Bits 20:22 - desc EXCKS"]
    #[inline(always)]
    pub fn excks(&mut self) -> ExcksW<'_, ScfgrSpec> {
        ExcksW::new(self, 20)
    }
    #[doc = "Bits 24:26 - desc HCLKS"]
    #[inline(always)]
    pub fn hclks(&mut self) -> HclksW<'_, ScfgrSpec> {
        HclksW::new(self, 24)
    }
}
#[doc = "desc SCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScfgrSpec;
impl crate::RegisterSpec for ScfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfgr::R`](R) reader structure"]
impl crate::Readable for ScfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`scfgr::W`](W) writer structure"]
impl crate::Writable for ScfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCFGR to value 0"]
impl crate::Resettable for ScfgrSpec {}
