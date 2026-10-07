#[doc = "Register `FCR` reader"]
pub type R = crate::R<FcrSpec>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `AWSL` reader - desc AWSL"]
pub type AwslR = crate::FieldReader;
#[doc = "Field `AWSL` writer - desc AWSL"]
pub type AwslW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FOUR_BIC` reader - desc FOUR_BIC"]
pub type FourBicR = crate::BitReader;
#[doc = "Field `FOUR_BIC` writer - desc FOUR_BIC"]
pub type FourBicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSNHD` reader - desc SSNHD"]
pub type SsnhdR = crate::BitReader;
#[doc = "Field `SSNHD` writer - desc SSNHD"]
pub type SsnhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSNLD` reader - desc SSNLD"]
pub type SsnldR = crate::BitReader;
#[doc = "Field `SSNLD` writer - desc SSNLD"]
pub type SsnldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPOL` reader - desc WPOL"]
pub type WpolR = crate::BitReader;
#[doc = "Field `WPOL` writer - desc WPOL"]
pub type WpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMCYCN` reader - desc DMCYCN"]
pub type DmcycnR = crate::FieldReader;
#[doc = "Field `DMCYCN` writer - desc DMCYCN"]
pub type DmcycnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DUTY` reader - desc DUTY"]
pub type DutyR = crate::BitReader;
#[doc = "Field `DUTY` writer - desc DUTY"]
pub type DutyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc AWSL"]
    #[inline(always)]
    pub fn awsl(&self) -> AwslR {
        AwslR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc FOUR_BIC"]
    #[inline(always)]
    pub fn four_bic(&self) -> FourBicR {
        FourBicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SSNHD"]
    #[inline(always)]
    pub fn ssnhd(&self) -> SsnhdR {
        SsnhdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc SSNLD"]
    #[inline(always)]
    pub fn ssnld(&self) -> SsnldR {
        SsnldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc WPOL"]
    #[inline(always)]
    pub fn wpol(&self) -> WpolR {
        WpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc DMCYCN"]
    #[inline(always)]
    pub fn dmcycn(&self) -> DmcycnR {
        DmcycnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - desc DUTY"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc AWSL"]
    #[inline(always)]
    pub fn awsl(&mut self) -> AwslW<'_, FcrSpec> {
        AwslW::new(self, 0)
    }
    #[doc = "Bit 2 - desc FOUR_BIC"]
    #[inline(always)]
    pub fn four_bic(&mut self) -> FourBicW<'_, FcrSpec> {
        FourBicW::new(self, 2)
    }
    #[doc = "Bit 4 - desc SSNHD"]
    #[inline(always)]
    pub fn ssnhd(&mut self) -> SsnhdW<'_, FcrSpec> {
        SsnhdW::new(self, 4)
    }
    #[doc = "Bit 5 - desc SSNLD"]
    #[inline(always)]
    pub fn ssnld(&mut self) -> SsnldW<'_, FcrSpec> {
        SsnldW::new(self, 5)
    }
    #[doc = "Bit 6 - desc WPOL"]
    #[inline(always)]
    pub fn wpol(&mut self) -> WpolW<'_, FcrSpec> {
        WpolW::new(self, 6)
    }
    #[doc = "Bits 8:11 - desc DMCYCN"]
    #[inline(always)]
    pub fn dmcycn(&mut self) -> DmcycnW<'_, FcrSpec> {
        DmcycnW::new(self, 8)
    }
    #[doc = "Bit 15 - desc DUTY"]
    #[inline(always)]
    pub fn duty(&mut self) -> DutyW<'_, FcrSpec> {
        DutyW::new(self, 15)
    }
}
#[doc = "desc FCR\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCR to value 0x80b3"]
impl crate::Resettable for FcrSpec {
    const RESET_VALUE: u32 = 0x80b3;
}
