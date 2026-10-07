#[doc = "Register `DIEPEMPMSK` reader"]
pub type R = crate::R<DiepempmskSpec>;
#[doc = "Register `DIEPEMPMSK` writer"]
pub type W = crate::W<DiepempmskSpec>;
#[doc = "Field `INEPTXFEM` reader - desc INEPTXFEM"]
pub type IneptxfemR = crate::FieldReader;
#[doc = "Field `INEPTXFEM` writer - desc INEPTXFEM"]
pub type IneptxfemW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - desc INEPTXFEM"]
    #[inline(always)]
    pub fn ineptxfem(&self) -> IneptxfemR {
        IneptxfemR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc INEPTXFEM"]
    #[inline(always)]
    pub fn ineptxfem(&mut self) -> IneptxfemW<'_, DiepempmskSpec> {
        IneptxfemW::new(self, 0)
    }
}
#[doc = "desc DIEPEMPMSK\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepempmskSpec;
impl crate::RegisterSpec for DiepempmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepempmsk::R`](R) reader structure"]
impl crate::Readable for DiepempmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepempmsk::W`](W) writer structure"]
impl crate::Writable for DiepempmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DiepempmskSpec {}
