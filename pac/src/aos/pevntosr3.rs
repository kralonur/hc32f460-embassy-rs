#[doc = "Register `PEVNTOSR3` reader"]
pub type R = crate::R<Pevntosr3Spec>;
#[doc = "Register `PEVNTOSR3` writer"]
pub type W = crate::W<Pevntosr3Spec>;
#[doc = "Field `POS` reader - desc POS"]
pub type PosR = crate::FieldReader<u16>;
#[doc = "Field `POS` writer - desc POS"]
pub type PosW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc POS"]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc POS"]
    #[inline(always)]
    pub fn pos(&mut self) -> PosW<'_, Pevntosr3Spec> {
        PosW::new(self, 0)
    }
}
#[doc = "desc PEVNTOSR3\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntosr3Spec;
impl crate::RegisterSpec for Pevntosr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntosr3::R`](R) reader structure"]
impl crate::Readable for Pevntosr3Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntosr3::W`](W) writer structure"]
impl crate::Writable for Pevntosr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTOSR3 to value 0"]
impl crate::Resettable for Pevntosr3Spec {}
