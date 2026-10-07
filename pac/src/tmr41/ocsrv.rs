#[doc = "Register `OCSRV` reader"]
pub type R = crate::R<OcsrvSpec>;
#[doc = "Register `OCSRV` writer"]
pub type W = crate::W<OcsrvSpec>;
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
    pub fn oceh(&mut self) -> OcehW<'_, OcsrvSpec> {
        OcehW::new(self, 0)
    }
    #[doc = "Bit 1 - desc OCEL"]
    #[inline(always)]
    pub fn ocel(&mut self) -> OcelW<'_, OcsrvSpec> {
        OcelW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OCPH"]
    #[inline(always)]
    pub fn ocph(&mut self) -> OcphW<'_, OcsrvSpec> {
        OcphW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OCPL"]
    #[inline(always)]
    pub fn ocpl(&mut self) -> OcplW<'_, OcsrvSpec> {
        OcplW::new(self, 3)
    }
    #[doc = "Bit 4 - desc OCIEH"]
    #[inline(always)]
    pub fn ocieh(&mut self) -> OciehW<'_, OcsrvSpec> {
        OciehW::new(self, 4)
    }
    #[doc = "Bit 5 - desc OCIEL"]
    #[inline(always)]
    pub fn ociel(&mut self) -> OcielW<'_, OcsrvSpec> {
        OcielW::new(self, 5)
    }
    #[doc = "Bit 6 - desc OCFH"]
    #[inline(always)]
    pub fn ocfh(&mut self) -> OcfhW<'_, OcsrvSpec> {
        OcfhW::new(self, 6)
    }
    #[doc = "Bit 7 - desc OCFL"]
    #[inline(always)]
    pub fn ocfl(&mut self) -> OcflW<'_, OcsrvSpec> {
        OcflW::new(self, 7)
    }
}
#[doc = "desc OCSRV\n\nYou can [`read`](crate::Reg::read) this register and get [`ocsrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocsrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OcsrvSpec;
impl crate::RegisterSpec for OcsrvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ocsrv::R`](R) reader structure"]
impl crate::Readable for OcsrvSpec {}
#[doc = "`write(|w| ..)` method takes [`ocsrv::W`](W) writer structure"]
impl crate::Writable for OcsrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCSRV to value 0xff00"]
impl crate::Resettable for OcsrvSpec {
    const RESET_VALUE: u16 = 0xff00;
}
