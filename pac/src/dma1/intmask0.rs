#[doc = "Register `INTMASK0` reader"]
pub type R = crate::R<Intmask0Spec>;
#[doc = "Register `INTMASK0` writer"]
pub type W = crate::W<Intmask0Spec>;
#[doc = "Field `MSKTRNERR` reader - desc MSKTRNERR"]
pub type MsktrnerrR = crate::FieldReader;
#[doc = "Field `MSKTRNERR` writer - desc MSKTRNERR"]
pub type MsktrnerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSKREQERR` reader - desc MSKREQERR"]
pub type MskreqerrR = crate::FieldReader;
#[doc = "Field `MSKREQERR` writer - desc MSKREQERR"]
pub type MskreqerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc MSKTRNERR"]
    #[inline(always)]
    pub fn msktrnerr(&self) -> MsktrnerrR {
        MsktrnerrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - desc MSKREQERR"]
    #[inline(always)]
    pub fn mskreqerr(&self) -> MskreqerrR {
        MskreqerrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MSKTRNERR"]
    #[inline(always)]
    pub fn msktrnerr(&mut self) -> MsktrnerrW<'_, Intmask0Spec> {
        MsktrnerrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc MSKREQERR"]
    #[inline(always)]
    pub fn mskreqerr(&mut self) -> MskreqerrW<'_, Intmask0Spec> {
        MskreqerrW::new(self, 16)
    }
}
#[doc = "desc INTMASK0\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intmask0Spec;
impl crate::RegisterSpec for Intmask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask0::R`](R) reader structure"]
impl crate::Readable for Intmask0Spec {}
#[doc = "`write(|w| ..)` method takes [`intmask0::W`](W) writer structure"]
impl crate::Writable for Intmask0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMASK0 to value 0"]
impl crate::Resettable for Intmask0Spec {}
