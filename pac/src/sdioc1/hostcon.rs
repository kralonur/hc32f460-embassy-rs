#[doc = "Register `HOSTCON` reader"]
pub type R = crate::R<HostconSpec>;
#[doc = "Register `HOSTCON` writer"]
pub type W = crate::W<HostconSpec>;
#[doc = "Field `DW` reader - desc DW"]
pub type DwR = crate::BitReader;
#[doc = "Field `DW` writer - desc DW"]
pub type DwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEN` reader - desc HSEN"]
pub type HsenR = crate::BitReader;
#[doc = "Field `HSEN` writer - desc HSEN"]
pub type HsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXDW` reader - desc EXDW"]
pub type ExdwR = crate::BitReader;
#[doc = "Field `EXDW` writer - desc EXDW"]
pub type ExdwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTL` reader - desc CDTL"]
pub type CdtlR = crate::BitReader;
#[doc = "Field `CDTL` writer - desc CDTL"]
pub type CdtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDSS` reader - desc CDSS"]
pub type CdssR = crate::BitReader;
#[doc = "Field `CDSS` writer - desc CDSS"]
pub type CdssW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc DW"]
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc HSEN"]
    #[inline(always)]
    pub fn hsen(&self) -> HsenR {
        HsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EXDW"]
    #[inline(always)]
    pub fn exdw(&self) -> ExdwR {
        ExdwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc CDTL"]
    #[inline(always)]
    pub fn cdtl(&self) -> CdtlR {
        CdtlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc CDSS"]
    #[inline(always)]
    pub fn cdss(&self) -> CdssR {
        CdssR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc DW"]
    #[inline(always)]
    pub fn dw(&mut self) -> DwW<'_, HostconSpec> {
        DwW::new(self, 1)
    }
    #[doc = "Bit 2 - desc HSEN"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HsenW<'_, HostconSpec> {
        HsenW::new(self, 2)
    }
    #[doc = "Bit 5 - desc EXDW"]
    #[inline(always)]
    pub fn exdw(&mut self) -> ExdwW<'_, HostconSpec> {
        ExdwW::new(self, 5)
    }
    #[doc = "Bit 6 - desc CDTL"]
    #[inline(always)]
    pub fn cdtl(&mut self) -> CdtlW<'_, HostconSpec> {
        CdtlW::new(self, 6)
    }
    #[doc = "Bit 7 - desc CDSS"]
    #[inline(always)]
    pub fn cdss(&mut self) -> CdssW<'_, HostconSpec> {
        CdssW::new(self, 7)
    }
}
#[doc = "desc HOSTCON\n\nYou can [`read`](crate::Reg::read) this register and get [`hostcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hostcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostconSpec;
impl crate::RegisterSpec for HostconSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hostcon::R`](R) reader structure"]
impl crate::Readable for HostconSpec {}
#[doc = "`write(|w| ..)` method takes [`hostcon::W`](W) writer structure"]
impl crate::Writable for HostconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOSTCON to value 0"]
impl crate::Resettable for HostconSpec {}
