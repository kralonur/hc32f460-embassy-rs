#[doc = "Register `OCSRW` reader"]
pub type R = crate::R<OcsrwSpec>;
#[doc = "Register `OCSRW` writer"]
pub type W = crate::W<OcsrwSpec>;
#[doc = "Field `OCEH` reader - desc OCEH"]
pub type OcehR = crate::BitReader;
#[doc = "Field `OCEH` writer - desc OCEH"]
pub type OcehW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCEL` reader - desc OCEL"]
pub type OcelR = crate::BitReader;
#[doc = "Field `OCEL` writer - desc OCEL"]
pub type OcelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPH` reader - desc OCPH"]
pub type OcphR = crate::BitReader;
#[doc = "Field `OCPH` writer - desc OCPH"]
pub type OcphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPL` reader - desc OCPL"]
pub type OcplR = crate::BitReader;
#[doc = "Field `OCPL` writer - desc OCPL"]
pub type OcplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIEH` reader - desc OCIEH"]
pub type OciehR = crate::BitReader;
#[doc = "Field `OCIEH` writer - desc OCIEH"]
pub type OciehW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCIEL` reader - desc OCIEL"]
pub type OcielR = crate::BitReader;
#[doc = "Field `OCIEL` writer - desc OCIEL"]
pub type OcielW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFH` reader - desc OCFH"]
pub type OcfhR = crate::BitReader;
#[doc = "Field `OCFH` writer - desc OCFH"]
pub type OcfhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCFL` reader - desc OCFL"]
pub type OcflR = crate::BitReader;
#[doc = "Field `OCFL` writer - desc OCFL"]
pub type OcflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc OCEH"]
    #[inline(always)]
    pub fn oceh(&self) -> OcehR {
        OcehR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OCEL"]
    #[inline(always)]
    pub fn ocel(&self) -> OcelR {
        OcelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OCPH"]
    #[inline(always)]
    pub fn ocph(&self) -> OcphR {
        OcphR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OCPL"]
    #[inline(always)]
    pub fn ocpl(&self) -> OcplR {
        OcplR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OCIEH"]
    #[inline(always)]
    pub fn ocieh(&self) -> OciehR {
        OciehR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OCIEL"]
    #[inline(always)]
    pub fn ociel(&self) -> OcielR {
        OcielR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OCFH"]
    #[inline(always)]
    pub fn ocfh(&self) -> OcfhR {
        OcfhR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc OCFL"]
    #[inline(always)]
    pub fn ocfl(&self) -> OcflR {
        OcflR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OCEH"]
    #[inline(always)]
    pub fn oceh(&mut self) -> OcehW<'_, OcsrwSpec> {
        OcehW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OCEL"]
    #[inline(always)]
    pub fn ocel(&mut self) -> OcelW<'_, OcsrwSpec> {
        OcelW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OCPH"]
    #[inline(always)]
    pub fn ocph(&mut self) -> OcphW<'_, OcsrwSpec> {
        OcphW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OCPL"]
    #[inline(always)]
    pub fn ocpl(&mut self) -> OcplW<'_, OcsrwSpec> {
        OcplW::new(self, 3)
    }
    #[doc = "Bit 4 - desc OCIEH"]
    #[inline(always)]
    pub fn ocieh(&mut self) -> OciehW<'_, OcsrwSpec> {
        OciehW::new(self, 4)
    }
    #[doc = "Bit 5 - desc OCIEL"]
    #[inline(always)]
    pub fn ociel(&mut self) -> OcielW<'_, OcsrwSpec> {
        OcielW::new(self, 5)
    }
    #[doc = "Bit 6 - desc OCFH"]
    #[inline(always)]
    pub fn ocfh(&mut self) -> OcfhW<'_, OcsrwSpec> {
        OcfhW::new(self, 6)
    }
    #[doc = "Bit 7 - desc OCFL"]
    #[inline(always)]
    pub fn ocfl(&mut self) -> OcflW<'_, OcsrwSpec> {
        OcflW::new(self, 7)
    }
}
#[doc = "desc OCSRW\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcsrwSpec;
impl crate::RegisterSpec for OcsrwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocsrw::R`](R) reader structure"]
impl crate::Readable for OcsrwSpec {}
#[doc = "`write(|w| ..)` method takes [`ocsrw::W`](W) writer structure"]
impl crate::Writable for OcsrwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCSRW to value 0xff00"]
impl crate::Resettable for OcsrwSpec {
    const RESET_VALUE: u16 = 0xff00;
}
