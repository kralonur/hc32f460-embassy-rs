#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GusbcfgSpec>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GusbcfgSpec>;
#[doc = "Field `TOCAL` reader - desc TOCAL"]
pub type TocalR = crate::FieldReader;
#[doc = "Field `TOCAL` writer - desc TOCAL"]
pub type TocalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYSEL` reader - desc PHYSEL"]
pub type PhyselR = crate::BitReader;
#[doc = "Field `PHYSEL` writer - desc PHYSEL"]
pub type PhyselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRDT` reader - desc TRDT"]
pub type TrdtR = crate::FieldReader;
#[doc = "Field `TRDT` writer - desc TRDT"]
pub type TrdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FHMOD` reader - desc FHMOD"]
pub type FhmodR = crate::BitReader;
#[doc = "Field `FHMOD` writer - desc FHMOD"]
pub type FhmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOD` reader - desc FDMOD"]
pub type FdmodR = crate::BitReader;
#[doc = "Field `FDMOD` writer - desc FDMOD"]
pub type FdmodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc TOCAL"]
    #[inline(always)]
    pub fn tocal(&self) -> TocalR {
        TocalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - desc PHYSEL"]
    #[inline(always)]
    pub fn physel(&self) -> PhyselR {
        PhyselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:13 - desc TRDT"]
    #[inline(always)]
    pub fn trdt(&self) -> TrdtR {
        TrdtR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - desc FHMOD"]
    #[inline(always)]
    pub fn fhmod(&self) -> FhmodR {
        FhmodR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc FDMOD"]
    #[inline(always)]
    pub fn fdmod(&self) -> FdmodR {
        FdmodR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc TOCAL"]
    #[inline(always)]
    pub fn tocal(&mut self) -> TocalW<'_, GusbcfgSpec> {
        TocalW::new(self, 0)
    }
    #[doc = "Bit 6 - desc PHYSEL"]
    #[inline(always)]
    pub fn physel(&mut self) -> PhyselW<'_, GusbcfgSpec> {
        PhyselW::new(self, 6)
    }
    #[doc = "Bits 10:13 - desc TRDT"]
    #[inline(always)]
    pub fn trdt(&mut self) -> TrdtW<'_, GusbcfgSpec> {
        TrdtW::new(self, 10)
    }
    #[doc = "Bit 29 - desc FHMOD"]
    #[inline(always)]
    pub fn fhmod(&mut self) -> FhmodW<'_, GusbcfgSpec> {
        FhmodW::new(self, 29)
    }
    #[doc = "Bit 30 - desc FDMOD"]
    #[inline(always)]
    pub fn fdmod(&mut self) -> FdmodW<'_, GusbcfgSpec> {
        FdmodW::new(self, 30)
    }
}
#[doc = "desc GUSBCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcfgSpec;
impl crate::RegisterSpec for GusbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GusbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GusbcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GusbcfgSpec {
    const RESET_VALUE: u32 = 0x0a00;
}
