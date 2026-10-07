#[doc = "Register `VPERR` reader"]
pub type R = crate::R<VperrSpec>;
#[doc = "Register `VPERR` writer"]
pub type W = crate::W<VperrSpec>;
#[doc = "Field `SPPERIA` reader - desc SPPERIA"]
pub type SpperiaR = crate::BitReader;
#[doc = "Field `SPPERIA` writer - desc SPPERIA"]
pub type SpperiaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPPERIB` reader - desc SPPERIB"]
pub type SpperibR = crate::BitReader;
#[doc = "Field `SPPERIB` writer - desc SPPERIB"]
pub type SpperibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNTE` reader - desc PCNTE"]
pub type PcnteR = crate::FieldReader;
#[doc = "Field `PCNTE` writer - desc PCNTE"]
pub type PcnteW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCNTS` reader - desc PCNTS"]
pub type PcntsR = crate::FieldReader;
#[doc = "Field `PCNTS` writer - desc PCNTS"]
pub type PcntsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 8 - desc SPPERIA"]
    #[inline(always)]
    pub fn spperia(&self) -> SpperiaR {
        SpperiaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SPPERIB"]
    #[inline(always)]
    pub fn spperib(&self) -> SpperibR {
        SpperibR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - desc PCNTE"]
    #[inline(always)]
    pub fn pcnte(&self) -> PcnteR {
        PcnteR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - desc PCNTS"]
    #[inline(always)]
    pub fn pcnts(&self) -> PcntsR {
        PcntsR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - desc SPPERIA"]
    #[inline(always)]
    pub fn spperia(&mut self) -> SpperiaW<'_, VperrSpec> {
        SpperiaW::new(self, 8)
    }
    #[doc = "Bit 9 - desc SPPERIB"]
    #[inline(always)]
    pub fn spperib(&mut self) -> SpperibW<'_, VperrSpec> {
        SpperibW::new(self, 9)
    }
    #[doc = "Bits 16:17 - desc PCNTE"]
    #[inline(always)]
    pub fn pcnte(&mut self) -> PcnteW<'_, VperrSpec> {
        PcnteW::new(self, 16)
    }
    #[doc = "Bits 18:20 - desc PCNTS"]
    #[inline(always)]
    pub fn pcnts(&mut self) -> PcntsW<'_, VperrSpec> {
        PcntsW::new(self, 18)
    }
}
#[doc = "desc VPERR\n\nYou can [`read`](crate::Reg::read) this register and get [`vperr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vperr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VperrSpec;
impl crate::RegisterSpec for VperrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vperr::R`](R) reader structure"]
impl crate::Readable for VperrSpec {}
#[doc = "`write(|w| ..)` method takes [`vperr::W`](W) writer structure"]
impl crate::Writable for VperrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VPERR to value 0"]
impl crate::Resettable for VperrSpec {}
