#[doc = "Register `CKPR` reader"]
pub type R = crate::R<CkprSpec>;
#[doc = "Register `CKPR` writer"]
pub type W = crate::W<CkprSpec>;
#[doc = "Field `CKPRC` reader - desc CKPRC"]
pub type CkprcR = crate::BitReader;
#[doc = "Field `CKPRC` writer - desc CKPRC"]
pub type CkprcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPRKW` reader - desc CKPRKW"]
pub type CkprkwR = crate::FieldReader;
#[doc = "Field `CKPRKW` writer - desc CKPRKW"]
pub type CkprkwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - desc CKPRC"]
    #[inline(always)]
    pub fn ckprc(&self) -> CkprcR {
        CkprcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - desc CKPRKW"]
    #[inline(always)]
    pub fn ckprkw(&self) -> CkprkwR {
        CkprkwR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc CKPRC"]
    #[inline(always)]
    pub fn ckprc(&mut self) -> CkprcW<'_, CkprSpec> {
        CkprcW::new(self, 0)
    }
    #[doc = "Bits 1:7 - desc CKPRKW"]
    #[inline(always)]
    pub fn ckprkw(&mut self) -> CkprkwW<'_, CkprSpec> {
        CkprkwW::new(self, 1)
    }
}
#[doc = "desc CKPR\n\nYou can [`read`](crate::Reg::read) this register and get [`ckpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkprSpec;
impl crate::RegisterSpec for CkprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckpr::R`](R) reader structure"]
impl crate::Readable for CkprSpec {}
#[doc = "`write(|w| ..)` method takes [`ckpr::W`](W) writer structure"]
impl crate::Writable for CkprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKPR to value 0"]
impl crate::Resettable for CkprSpec {}
