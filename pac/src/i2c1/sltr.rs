#[doc = "Register `SLTR` reader"]
pub type R = crate::R<SltrSpec>;
#[doc = "Register `SLTR` writer"]
pub type W = crate::W<SltrSpec>;
#[doc = "Field `TOUTLOW` reader - desc TOUTLOW"]
pub type ToutlowR = crate::FieldReader<u16>;
#[doc = "Field `TOUTLOW` writer - desc TOUTLOW"]
pub type ToutlowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUTHIGH` reader - desc TOUTHIGH"]
pub type TouthighR = crate::FieldReader<u16>;
#[doc = "Field `TOUTHIGH` writer - desc TOUTHIGH"]
pub type TouthighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc TOUTLOW"]
    #[inline(always)]
    pub fn toutlow(&self) -> ToutlowR {
        ToutlowR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc TOUTHIGH"]
    #[inline(always)]
    pub fn touthigh(&self) -> TouthighR {
        TouthighR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc TOUTLOW"]
    #[inline(always)]
    pub fn toutlow(&mut self) -> ToutlowW<'_, SltrSpec> {
        ToutlowW::new(self, 0)
    }
    #[doc = "Bits 16:31 - desc TOUTHIGH"]
    #[inline(always)]
    pub fn touthigh(&mut self) -> TouthighW<'_, SltrSpec> {
        TouthighW::new(self, 16)
    }
}
#[doc = "desc SLTR\n\nYou can [`read`](crate::Reg::read) this register and get [`sltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SltrSpec;
impl crate::RegisterSpec for SltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sltr::R`](R) reader structure"]
impl crate::Readable for SltrSpec {}
#[doc = "`write(|w| ..)` method takes [`sltr::W`](W) writer structure"]
impl crate::Writable for SltrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLTR to value 0xffff_ffff"]
impl crate::Resettable for SltrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
