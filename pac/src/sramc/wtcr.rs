#[doc = "Register `WTCR` reader"]
pub type R = crate::R<WtcrSpec>;
#[doc = "Register `WTCR` writer"]
pub type W = crate::W<WtcrSpec>;
#[doc = "Field `SRAM12_RWT` reader - desc SRAM12_RWT"]
pub type Sram12RwtR = crate::FieldReader;
#[doc = "Field `SRAM12_RWT` writer - desc SRAM12_RWT"]
pub type Sram12RwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAM12_WWT` reader - desc SRAM12_WWT"]
pub type Sram12WwtR = crate::FieldReader;
#[doc = "Field `SRAM12_WWT` writer - desc SRAM12_WWT"]
pub type Sram12WwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAM3_RWT` reader - desc SRAM3_RWT"]
pub type Sram3RwtR = crate::FieldReader;
#[doc = "Field `SRAM3_RWT` writer - desc SRAM3_RWT"]
pub type Sram3RwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAM3_WWT` reader - desc SRAM3_WWT"]
pub type Sram3WwtR = crate::FieldReader;
#[doc = "Field `SRAM3_WWT` writer - desc SRAM3_WWT"]
pub type Sram3WwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAMH_RWT` reader - desc SRAMH_RWT"]
pub type SramhRwtR = crate::FieldReader;
#[doc = "Field `SRAMH_RWT` writer - desc SRAMH_RWT"]
pub type SramhRwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAMH_WWT` reader - desc SRAMH_WWT"]
pub type SramhWwtR = crate::FieldReader;
#[doc = "Field `SRAMH_WWT` writer - desc SRAMH_WWT"]
pub type SramhWwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAMR_RWT` reader - desc SRAMR_RWT"]
pub type SramrRwtR = crate::FieldReader;
#[doc = "Field `SRAMR_RWT` writer - desc SRAMR_RWT"]
pub type SramrRwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAMR_WWT` reader - desc SRAMR_WWT"]
pub type SramrWwtR = crate::FieldReader;
#[doc = "Field `SRAMR_WWT` writer - desc SRAMR_WWT"]
pub type SramrWwtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc SRAM12_RWT"]
    #[inline(always)]
    pub fn sram12_rwt(&self) -> Sram12RwtR {
        Sram12RwtR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc SRAM12_WWT"]
    #[inline(always)]
    pub fn sram12_wwt(&self) -> Sram12WwtR {
        Sram12WwtR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc SRAM3_RWT"]
    #[inline(always)]
    pub fn sram3_rwt(&self) -> Sram3RwtR {
        Sram3RwtR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc SRAM3_WWT"]
    #[inline(always)]
    pub fn sram3_wwt(&self) -> Sram3WwtR {
        Sram3WwtR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - desc SRAMH_RWT"]
    #[inline(always)]
    pub fn sramh_rwt(&self) -> SramhRwtR {
        SramhRwtR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - desc SRAMH_WWT"]
    #[inline(always)]
    pub fn sramh_wwt(&self) -> SramhWwtR {
        SramhWwtR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - desc SRAMR_RWT"]
    #[inline(always)]
    pub fn sramr_rwt(&self) -> SramrRwtR {
        SramrRwtR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - desc SRAMR_WWT"]
    #[inline(always)]
    pub fn sramr_wwt(&self) -> SramrWwtR {
        SramrWwtR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc SRAM12_RWT"]
    #[inline(always)]
    pub fn sram12_rwt(&mut self) -> Sram12RwtW<'_, WtcrSpec> {
        Sram12RwtW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc SRAM12_WWT"]
    #[inline(always)]
    pub fn sram12_wwt(&mut self) -> Sram12WwtW<'_, WtcrSpec> {
        Sram12WwtW::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc SRAM3_RWT"]
    #[inline(always)]
    pub fn sram3_rwt(&mut self) -> Sram3RwtW<'_, WtcrSpec> {
        Sram3RwtW::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc SRAM3_WWT"]
    #[inline(always)]
    pub fn sram3_wwt(&mut self) -> Sram3WwtW<'_, WtcrSpec> {
        Sram3WwtW::new(self, 12)
    }
    #[doc = "Bits 16:18 - desc SRAMH_RWT"]
    #[inline(always)]
    pub fn sramh_rwt(&mut self) -> SramhRwtW<'_, WtcrSpec> {
        SramhRwtW::new(self, 16)
    }
    #[doc = "Bits 20:22 - desc SRAMH_WWT"]
    #[inline(always)]
    pub fn sramh_wwt(&mut self) -> SramhWwtW<'_, WtcrSpec> {
        SramhWwtW::new(self, 20)
    }
    #[doc = "Bits 24:26 - desc SRAMR_RWT"]
    #[inline(always)]
    pub fn sramr_rwt(&mut self) -> SramrRwtW<'_, WtcrSpec> {
        SramrRwtW::new(self, 24)
    }
    #[doc = "Bits 28:30 - desc SRAMR_WWT"]
    #[inline(always)]
    pub fn sramr_wwt(&mut self) -> SramrWwtW<'_, WtcrSpec> {
        SramrWwtW::new(self, 28)
    }
}
#[doc = "desc WTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`wtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtcrSpec;
impl crate::RegisterSpec for WtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtcr::R`](R) reader structure"]
impl crate::Readable for WtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wtcr::W`](W) writer structure"]
impl crate::Writable for WtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WTCR to value 0"]
impl crate::Resettable for WtcrSpec {}
