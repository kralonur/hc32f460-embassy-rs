#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `EOCAIEN` reader - desc EOCAIEN"]
pub type EocaienR = crate::BitReader;
#[doc = "Field `EOCAIEN` writer - desc EOCAIEN"]
pub type EocaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCBIEN` reader - desc EOCBIEN"]
pub type EocbienR = crate::BitReader;
#[doc = "Field `EOCBIEN` writer - desc EOCBIEN"]
pub type EocbienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EOCAIEN"]
    #[inline(always)]
    pub fn eocaien(&self) -> EocaienR {
        EocaienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOCBIEN"]
    #[inline(always)]
    pub fn eocbien(&self) -> EocbienR {
        EocbienR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EOCAIEN"]
    #[inline(always)]
    pub fn eocaien(&mut self) -> EocaienW<'_, IcrSpec> {
        EocaienW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EOCBIEN"]
    #[inline(always)]
    pub fn eocbien(&mut self) -> EocbienW<'_, IcrSpec> {
        EocbienW::new(self, 1)
    }
}
#[doc = "desc ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
