#[doc = "Register `PEVNTDIRR4` reader"]
pub type R = crate::R<Pevntdirr4Spec>;
#[doc = "Register `PEVNTDIRR4` writer"]
pub type W = crate::W<Pevntdirr4Spec>;
#[doc = "Field `PDIR` reader - desc PDIR"]
pub type PdirR = crate::FieldReader<u16>;
#[doc = "Field `PDIR` writer - desc PDIR"]
pub type PdirW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc PDIR"]
    #[inline(always)]
    pub fn pdir(&self) -> PdirR {
        PdirR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PDIR"]
    #[inline(always)]
    pub fn pdir(&mut self) -> PdirW<'_, Pevntdirr4Spec> {
        PdirW::new(self, 0)
    }
}
#[doc = "desc PEVNTDIRR4\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntdirr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntdirr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntdirr4Spec;
impl crate::RegisterSpec for Pevntdirr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntdirr4::R`](R) reader structure"]
impl crate::Readable for Pevntdirr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntdirr4::W`](W) writer structure"]
impl crate::Writable for Pevntdirr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTDIRR4 to value 0"]
impl crate::Resettable for Pevntdirr4Spec {}
