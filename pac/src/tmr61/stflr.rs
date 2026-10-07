#[doc = "Register `STFLR` reader"]
pub type R = crate::R<StflrSpec>;
#[doc = "Register `STFLR` writer"]
pub type W = crate::W<StflrSpec>;
#[doc = "Field `CMAF` reader - desc CMAF"]
pub type CmafR = crate::BitReader;
#[doc = "Field `CMAF` writer - desc CMAF"]
pub type CmafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMBF` reader - desc CMBF"]
pub type CmbfR = crate::BitReader;
#[doc = "Field `CMBF` writer - desc CMBF"]
pub type CmbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` reader - desc CMCF"]
pub type CmcfR = crate::BitReader;
#[doc = "Field `CMCF` writer - desc CMCF"]
pub type CmcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDF` reader - desc CMDF"]
pub type CmdfR = crate::BitReader;
#[doc = "Field `CMDF` writer - desc CMDF"]
pub type CmdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMEF` reader - desc CMEF"]
pub type CmefR = crate::BitReader;
#[doc = "Field `CMEF` writer - desc CMEF"]
pub type CmefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMFF` reader - desc CMFF"]
pub type CmffR = crate::BitReader;
#[doc = "Field `CMFF` writer - desc CMFF"]
pub type CmffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFF` reader - desc OVFF"]
pub type OvffR = crate::BitReader;
#[doc = "Field `OVFF` writer - desc OVFF"]
pub type OvffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDFF` reader - desc UDFF"]
pub type UdffR = crate::BitReader;
#[doc = "Field `UDFF` writer - desc UDFF"]
pub type UdffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEF` reader - desc DTEF"]
pub type DtefR = crate::BitReader;
#[doc = "Field `CMSAUF` reader - desc CMSAUF"]
pub type CmsaufR = crate::BitReader;
#[doc = "Field `CMSAUF` writer - desc CMSAUF"]
pub type CmsaufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMSADF` reader - desc CMSADF"]
pub type CmsadfR = crate::BitReader;
#[doc = "Field `CMSADF` writer - desc CMSADF"]
pub type CmsadfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMSBUF` reader - desc CMSBUF"]
pub type CmsbufR = crate::BitReader;
#[doc = "Field `CMSBUF` writer - desc CMSBUF"]
pub type CmsbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMSBDF` reader - desc CMSBDF"]
pub type CmsbdfR = crate::BitReader;
#[doc = "Field `CMSBDF` writer - desc CMSBDF"]
pub type CmsbdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPERNUM` reader - desc VPERNUM"]
pub type VpernumR = crate::FieldReader;
#[doc = "Field `DIRF` reader - desc DIRF"]
pub type DirfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc CMAF"]
    #[inline(always)]
    pub fn cmaf(&self) -> CmafR {
        CmafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMBF"]
    #[inline(always)]
    pub fn cmbf(&self) -> CmbfR {
        CmbfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMCF"]
    #[inline(always)]
    pub fn cmcf(&self) -> CmcfR {
        CmcfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMDF"]
    #[inline(always)]
    pub fn cmdf(&self) -> CmdfR {
        CmdfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CMEF"]
    #[inline(always)]
    pub fn cmef(&self) -> CmefR {
        CmefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CMFF"]
    #[inline(always)]
    pub fn cmff(&self) -> CmffR {
        CmffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVFF"]
    #[inline(always)]
    pub fn ovff(&self) -> OvffR {
        OvffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&self) -> UdffR {
        UdffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc DTEF"]
    #[inline(always)]
    pub fn dtef(&self) -> DtefR {
        DtefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CMSAUF"]
    #[inline(always)]
    pub fn cmsauf(&self) -> CmsaufR {
        CmsaufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc CMSADF"]
    #[inline(always)]
    pub fn cmsadf(&self) -> CmsadfR {
        CmsadfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc CMSBUF"]
    #[inline(always)]
    pub fn cmsbuf(&self) -> CmsbufR {
        CmsbufR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc CMSBDF"]
    #[inline(always)]
    pub fn cmsbdf(&self) -> CmsbdfR {
        CmsbdfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 21:23 - desc VPERNUM"]
    #[inline(always)]
    pub fn vpernum(&self) -> VpernumR {
        VpernumR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 31 - desc DIRF"]
    #[inline(always)]
    pub fn dirf(&self) -> DirfR {
        DirfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CMAF"]
    #[inline(always)]
    pub fn cmaf(&mut self) -> CmafW<'_, StflrSpec> {
        CmafW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CMBF"]
    #[inline(always)]
    pub fn cmbf(&mut self) -> CmbfW<'_, StflrSpec> {
        CmbfW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMCF"]
    #[inline(always)]
    pub fn cmcf(&mut self) -> CmcfW<'_, StflrSpec> {
        CmcfW::new(self, 2)
    }
    #[doc = "Bit 3 - desc CMDF"]
    #[inline(always)]
    pub fn cmdf(&mut self) -> CmdfW<'_, StflrSpec> {
        CmdfW::new(self, 3)
    }
    #[doc = "Bit 4 - desc CMEF"]
    #[inline(always)]
    pub fn cmef(&mut self) -> CmefW<'_, StflrSpec> {
        CmefW::new(self, 4)
    }
    #[doc = "Bit 5 - desc CMFF"]
    #[inline(always)]
    pub fn cmff(&mut self) -> CmffW<'_, StflrSpec> {
        CmffW::new(self, 5)
    }
    #[doc = "Bit 6 - desc OVFF"]
    #[inline(always)]
    pub fn ovff(&mut self) -> OvffW<'_, StflrSpec> {
        OvffW::new(self, 6)
    }
    #[doc = "Bit 7 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&mut self) -> UdffW<'_, StflrSpec> {
        UdffW::new(self, 7)
    }
    #[doc = "Bit 9 - desc CMSAUF"]
    #[inline(always)]
    pub fn cmsauf(&mut self) -> CmsaufW<'_, StflrSpec> {
        CmsaufW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CMSADF"]
    #[inline(always)]
    pub fn cmsadf(&mut self) -> CmsadfW<'_, StflrSpec> {
        CmsadfW::new(self, 10)
    }
    #[doc = "Bit 11 - desc CMSBUF"]
    #[inline(always)]
    pub fn cmsbuf(&mut self) -> CmsbufW<'_, StflrSpec> {
        CmsbufW::new(self, 11)
    }
    #[doc = "Bit 12 - desc CMSBDF"]
    #[inline(always)]
    pub fn cmsbdf(&mut self) -> CmsbdfW<'_, StflrSpec> {
        CmsbdfW::new(self, 12)
    }
}
#[doc = "desc STFLR\n\nYou can [`read`](crate::Reg::read) this register and get [`stflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StflrSpec;
impl crate::RegisterSpec for StflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stflr::R`](R) reader structure"]
impl crate::Readable for StflrSpec {}
#[doc = "`write(|w| ..)` method takes [`stflr::W`](W) writer structure"]
impl crate::Writable for StflrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STFLR to value 0x8000_0000"]
impl crate::Resettable for StflrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
