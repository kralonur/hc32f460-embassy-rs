#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HcfgSpec>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HcfgSpec>;
#[doc = "Field `FSLSPCS` reader - desc FSLSPCS"]
pub type FslspcsR = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - desc FSLSPCS"]
pub type FslspcsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSS` reader - desc FSLSS"]
pub type FslssR = crate::BitReader;
#[doc = "Field `FSLSS` writer - desc FSLSS"]
pub type FslssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FslspcsR {
        FslspcsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - desc FSLSS"]
    #[inline(always)]
    pub fn fslss(&self) -> FslssR {
        FslssR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FslspcsW<'_, HcfgSpec> {
        FslspcsW::new(self, 0)
    }
    #[doc = "Bit 2 - desc FSLSS"]
    #[inline(always)]
    pub fn fslss(&mut self) -> FslssW<'_, HcfgSpec> {
        FslssW::new(self, 2)
    }
}
#[doc = "desc HCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfgSpec;
impl crate::RegisterSpec for HcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCFG to value 0x0020_0000"]
impl crate::Resettable for HcfgSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
