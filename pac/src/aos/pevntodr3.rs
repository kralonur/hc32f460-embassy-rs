#[doc = "Register `PEVNTODR3` reader"]
pub type R = crate::R<Pevntodr3Spec>;
#[doc = "Register `PEVNTODR3` writer"]
pub type W = crate::W<Pevntodr3Spec>;
#[doc = "Field `POUT` reader - desc POUT"]
pub type PoutR = crate::FieldReader<u16>;
#[doc = "Field `POUT` writer - desc POUT"]
pub type PoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc POUT"]
    #[inline(always)]
    pub fn pout(&self) -> PoutR {
        PoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc POUT"]
    #[inline(always)]
    pub fn pout(&mut self) -> PoutW<'_, Pevntodr3Spec> {
        PoutW::new(self, 0)
    }
}
#[doc = "desc PEVNTODR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntodr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntodr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntodr3Spec;
impl crate::RegisterSpec for Pevntodr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntodr3::R`](R) reader structure"]
impl crate::Readable for Pevntodr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntodr3::W`](W) writer structure"]
impl crate::Writable for Pevntodr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTODR3 to value 0"]
impl crate::Resettable for Pevntodr3Spec {}
